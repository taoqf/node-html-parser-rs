use std::collections::HashMap;

use crate::dom::element::HTMLElement;

// Instance-based adapter so we can carry flattened index metadata.
pub trait Adapter {
	type HTMLElement;
	// 基础节点判定
	fn is_tag(&self, node: &Self::HTMLElement) -> bool;
	fn get_children<'a>(&'a self, node: &'a Self::HTMLElement) -> Vec<&'a Self::HTMLElement>;
	fn get_parent<'a>(&'a self, node: &'a Self::HTMLElement) -> Option<&'a Self::HTMLElement>;
	fn get_siblings<'a>(&'a self, node: &'a Self::HTMLElement) -> Vec<&'a Self::HTMLElement>;
	fn get_name<'b>(&self, el: &'b Self::HTMLElement) -> &'b str;
	fn equals(&self, a: &Self::HTMLElement, b: &Self::HTMLElement) -> bool;
	fn get_attribute<'b>(&self, el: &'b Self::HTMLElement, name: &str) -> Option<&'b str>;
	// 附加：全部属性（供部分伪类或调试使用）
	fn all_attributes<'a>(&'a self, el: &'a Self::HTMLElement) -> Vec<(&'a str, &'a str)>;
	// 文本/空判断（:empty, :contains 等拓展需要；:contains 非标准暂不实现逻辑，仅预留）
	fn is_empty(&self, el: &Self::HTMLElement) -> bool;
}

pub struct FlatEntry<'a> {
	pub el: &'a HTMLElement,
	pub parent: Option<usize>,
	pub children: Vec<usize>,
}

pub struct FlatDom<'a> {
	pub entries: Vec<FlatEntry<'a>>,
	pub index: HashMap<*const HTMLElement, usize>,
}

impl<'a> FlatDom<'a> {
	pub fn build(root: &'a HTMLElement) -> Self {
		let mut entries: Vec<FlatEntry<'a>> = Vec::new();
		let mut index: HashMap<*const HTMLElement, usize> = HashMap::new();
		fn dfs<'b>(
			cur: &'b HTMLElement,
			parent: Option<usize>,
			entries: &mut Vec<FlatEntry<'b>>,
			index: &mut HashMap<*const HTMLElement, usize>,
		) -> usize {
			let id = entries.len();
			entries.push(FlatEntry {
				el: cur,
				parent,
				children: Vec::new(),
			});
			index.insert(cur as *const HTMLElement, id);
			// collect HTMLElement children
			let mut child_ids = Vec::new();
			for child in cur.iter_elements() {
				let cid = dfs(child, Some(id), entries, index);
				child_ids.push(cid);
			}
			entries[id].children = child_ids;
			id
		}
		dfs(root, None, &mut entries, &mut index);
		FlatDom { entries, index }
	}
	fn get_entry_by_el(&self, el: &HTMLElement) -> Option<&FlatEntry<'a>> {
		self.index
			.get(&(el as *const HTMLElement))
			.map(|i| &self.entries[*i])
	}
}

pub struct HtmlAdapter<'a> {
	pub flat: FlatDom<'a>,
}

impl<'a> HtmlAdapter<'a> {
	pub fn new(root: &'a HTMLElement) -> Self {
		Self {
			flat: FlatDom::build(root),
		}
	}
}

impl<'a> Adapter for HtmlAdapter<'a> {
	type HTMLElement = HTMLElement;
	fn is_tag(&self, _node: &Self::HTMLElement) -> bool {
		true
	}
	fn get_children<'b>(&'b self, node: &'b Self::HTMLElement) -> Vec<&'b Self::HTMLElement> {
		if let Some(entry) = self.flat.get_entry_by_el(node) {
			entry
				.children
				.iter()
				.map(|idx| self.flat.entries[*idx].el)
				.collect()
		} else {
			Vec::new()
		}
	}
	fn get_parent<'b>(&'b self, node: &'b Self::HTMLElement) -> Option<&'b Self::HTMLElement> {
		let entry = self.flat.get_entry_by_el(node)?;
		entry.parent.map(|pid| self.flat.entries[pid].el)
	}
	fn get_siblings<'b>(&'b self, node: &'b Self::HTMLElement) -> Vec<&'b Self::HTMLElement> {
		let entry = match self.flat.get_entry_by_el(node) {
			Some(e) => e,
			None => return Vec::new(),
		};
		match entry.parent {
			Some(pid) => self.flat.entries[pid]
				.children
				.iter()
				.map(|i| self.flat.entries[*i].el)
				.collect(),
			None => vec![entry.el],
		}
	}
	fn get_name<'b>(&self, el: &'b Self::HTMLElement) -> &'b str {
		el.name()
	}
	fn equals(&self, a: &Self::HTMLElement, b: &Self::HTMLElement) -> bool {
		std::ptr::eq(a, b)
	}
	fn get_attribute<'b>(&self, el: &'b Self::HTMLElement, name: &str) -> Option<&'b str> {
		// 确保补全所有属性（非惰性仅 id/class）以支持大小写无关匹配
		let mut_ptr = el as *const HTMLElement as *mut HTMLElement;
		unsafe {
			(*mut_ptr).ensure_all_attrs();
		}
		unsafe {
			for (k, v) in (*mut_ptr).attrs.iter() {
				if k.eq_ignore_ascii_case(name) {
					return Some(v.as_str());
				}
			}
		}
		None
	}
	fn all_attributes<'b>(&'b self, el: &'b Self::HTMLElement) -> Vec<(&'b str, &'b str)> {
		// 触发惰性补全
		let mut_ptr = el as *const HTMLElement as *mut HTMLElement;
		unsafe {
			(*mut_ptr).ensure_all_attrs();
		}
		unsafe {
			(*mut_ptr)
				.attrs
				.iter()
				.map(|(k, v)| (k.as_str(), v.as_str()))
				.collect()
		}
	}
	fn is_empty(&self, el: &Self::HTMLElement) -> bool {
		// 空：无元素子节点且无非空白文本
		for c in &el.children {
			match c {
				crate::dom::node::Node::Element(_) => return false,
				crate::dom::node::Node::Text(t) => {
					if !t.raw.trim().is_empty() {
						return false;
					}
				}
				crate::dom::node::Node::Comment(_) => {}
			}
		}
		true
	}
}

pub type Predicate<E> = dyn Fn(&E) -> bool;

pub struct CompiledQuery<'a, A: Adapter> {
	pub func: Box<dyn Fn(&A::HTMLElement) -> bool + 'a>,
	pub should_test_next_siblings: bool,
}

impl<'a, A: Adapter> CompiledQuery<'a, A> {
	pub fn new<F: Fn(&A::HTMLElement) -> bool + 'a>(f: F) -> Self {
		Self {
			func: Box::new(f),
			should_test_next_siblings: false,
		}
	}
	pub fn test(&self, el: &A::HTMLElement) -> bool {
		(self.func)(el)
	}
}

pub struct Options<A: Adapter> {
	pub xml_mode: bool,
	pub relative_selector: bool,
	pub cache_results: bool,
	pub lower_case_attribute_names: bool,
	pub lower_case_tags: bool,
	pub adapter: std::marker::PhantomData<A>,
}

impl<A: Adapter> Default for Options<A> {
	fn default() -> Self {
		Self {
			xml_mode: false,
			relative_selector: true,
			cache_results: true,
			lower_case_attribute_names: true,
			lower_case_tags: true,
			adapter: std::marker::PhantomData,
		}
	}
}

pub struct InternalOptions<A: Adapter> {
	pub xml_mode: bool,
	pub relative_selector: bool,
	pub cache_results: bool,
	pub lower_case_attribute_names: bool,
	pub lower_case_tags: bool,
	pub adapter: std::marker::PhantomData<A>,
}

impl<A: Adapter> From<&Options<A>> for InternalOptions<A> {
	fn from(o: &Options<A>) -> Self {
		Self {
			xml_mode: o.xml_mode,
			relative_selector: o.relative_selector,
			cache_results: o.cache_results,
			lower_case_attribute_names: o.lower_case_attribute_names,
			lower_case_tags: o.lower_case_tags,
			adapter: std::marker::PhantomData,
		}
	}
}

// --------------------------------------------------
// Selector related internal representations (Step 1)
// --------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeAction {
	Exists,
	Equals,
	Not,
	Start,
	End,
	Any,
	Hyphen,
	Element,
}

#[derive(Debug, Clone)]
pub struct AttributeSelector {
	pub name: String,
	pub action: AttributeAction,
	pub value: Option<String>,
	pub ignore_case: bool,
}

#[derive(Debug, Clone)]
pub enum PseudoData {
	None,
	SubSelectors(Vec<Vec<InternalSelector>>),
	Nth(crate::css_select::legacy::NthExpr),
}

#[derive(Debug, Clone)]
pub enum InternalSelector {
	// Traversal
	Descendant,
	Child,
	Sibling,
	Adjacent,
	FlexibleDescendant,
	// Basic
	Universal,
	Tag { name: String },
	Attribute(AttributeSelector),
	Pseudo { name: String, data: PseudoData },
}
