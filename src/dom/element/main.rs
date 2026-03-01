use super::content::parse_fragment;
use crate::dom::{node::Node, text::TextNode};
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct HTMLElement {
	pub(super) tag_name: Option<String>, // None for root container
	pub(crate) raw_attrs: String, // original attribute string (escaped quoting preserved as much as possible)
	pub attrs: Vec<(String, String)>, // lower-case key, decoded value
	pub children: Vec<Node>,
	pub(crate) parent: Option<*mut HTMLElement>,
	// Whether this element is a void element (no closing tag) according to options at parse time
	pub(super) is_void: bool,
	// Whether serializer should append a closing slash (<br/>)
	pub(super) void_add_slash: bool,
	// caches for JS-style attribute APIs
	pub(super) cache_raw_map: Option<HashMap<String, String>>, // original key -> raw (un-decoded) value or empty
	pub(super) cache_lower_decoded: Option<HashMap<String, String>>, // lowercase key -> decoded
	pub id: String,
	pub(super) class_cache: Option<Vec<String>>, // lazily parsed class tokens
	pub(super) range: Option<(usize, usize)>,    // (start,end)
	// 是否已完整解析所有 attrs（延迟解析机制预留，当前解析器初始阶段可只解析部分如 id/class）
	pub(crate) attrs_complete: bool,
	// 是否属性已被修改（用于决定序列化时是否需要标准化引号）
	pub(crate) attrs_modified: bool,
	pub(crate) parse_comment: bool,
	pub(crate) parse_lowercase: bool,
}

impl HTMLElement {
	pub fn new(
		tag: Option<String>,
		raw_attrs: String,
		attrs: Vec<(String, String)>,
		is_void: bool,
		void_add_slash: bool,
	) -> Self {
		// derive id from provided attrs vector if present (parity with JS ctor behavior #112)
		let mut id_val = String::new();
		for (k, v) in &attrs {
			if k.eq_ignore_ascii_case("id") {
				id_val = v.clone();
				break;
			}
		}
		Self {
			tag_name: tag,
			raw_attrs,
			attrs,
			// 🚀 优化：预分配children容量，减少重新分配
			children: Vec::with_capacity(2),
			parent: None,
			is_void,
			void_add_slash,
			cache_raw_map: None,
			cache_lower_decoded: None,

			id: id_val,
			class_cache: None,
			range: None, // will set to Some((-1,-1)) for non-root below
			attrs_complete: false,
			attrs_modified: false,
			parse_comment: false,
			parse_lowercase: false,
		}
	}
	// adopt_child（原 JS 未显式暴露；之前内部使用计划，现逻辑内联后移除）
	pub fn is_root(&self) -> bool {
		self.tag_name.is_none()
	}
	pub fn name(&self) -> &str {
		self.tag_name.as_deref().unwrap_or("")
	}
	/// JS HTMLElement.tagName setter 行为：赋值后序列化使用小写（JS 内部存 rawTagName 小写，tagName getter 返回大写）。
	/// 为贴近 JS，我们内部沿用小写存储，外部序列化 already 调用 self.name()（即原样）。
	pub fn set_tag_name(&mut self, new_name: &str) {
		let lowered = new_name.to_lowercase();
		self.tag_name = Some(lowered);
	}

	// classList like helpers
	pub fn raw_text(&self) -> String {
		// JS 行为：如果是 <br> 则 rawText 为 "\n"
		if !self.is_root() && self.name().eq_ignore_ascii_case("br") {
			return "\n".to_string();
		}
		let mut buf = String::new();
		for c in &self.children {
			buf.push_str(&c.raw_text());
		}
		buf
	}

	pub fn class_names(&self) -> String {
		self.get_attr("class").unwrap_or("").to_string()
	}
	pub fn inner_html(&self) -> String {
		self.children.iter().map(|c| c.to_html()).collect()
	}
	/// 设置 innerHTML：清空旧子节点并以解析后的片段替换
	pub fn set_inner_html(&mut self, html: &str) {
		let mut nodes = parse_fragment(html);
		if nodes.is_empty() {
			// JS: 若解析后没有子节点，则使用一个 TextNode(content) 占位
			nodes.push(Node::Text(TextNode::new(html.to_string())));
		}
		self.children.clear();
		let self_ptr: *mut HTMLElement = self as *mut HTMLElement;
		for n in nodes.iter_mut() {
			if let Node::Element(e) = n {
				e.parent = Some(self_ptr);
			}
		}
		self.children.extend(nodes);
	}

	// ---- Selector match & closest (模拟 JS HTMLElement.closest) ----
	/// 判断当前元素是否匹配 selector（使用全局选择再比对引用，性能次优）。
	pub fn matches_selector<'a>(&'a self, root: &'a HTMLElement, selector: &str) -> bool {
		// 利用已有 query_selector_all 从 root 选出全部匹配，再比较指针（与 JS Element.matches 行为等价）。
		let matches = root.query_selector_all(selector);
		let self_ptr = self as *const HTMLElement;
		matches.iter().any(|e| *e as *const HTMLElement == self_ptr)
	}
	/// JS Element.matches(selector)
	pub fn matches(&self, selector: &str) -> bool {
		let root = self.root();
		self.matches_selector(root, selector)
	}
	/// 获取当前树根元素（最外层容器）
	pub fn root(&self) -> &HTMLElement {
		let mut cur: &HTMLElement = self;
		while let Some(p) = cur.parent() {
			cur = p;
		}
		cur
	}
	/// JS closest(selector)
	pub fn closest(&self, selector: &str) -> Option<&HTMLElement> {
		let mut cur: Option<&HTMLElement> = Some(self);
		while let Some(c) = cur {
			if c.matches(selector) {
				return Some(c);
			}
			cur = c.parent();
		}
		None
	}
	/// JS clone()
	pub fn clone(&self) -> HTMLElement {
		self.clone_node()
	}

	pub fn iter_elements<'a>(&'a self) -> impl Iterator<Item = &'a HTMLElement> + 'a {
		self.children.iter().filter_map(|n| n.as_element())
	}
	pub fn query_selector_all<'a>(&'a self, selector: &str) -> Vec<&'a HTMLElement> {
		crate::css_select::select_all(selector, self)
	}
	pub fn query_selector<'a>(&'a self, selector: &str) -> Option<&'a HTMLElement> {
		self.query_selector_all(selector).into_iter().next()
	}

	pub fn remove_whitespace(&mut self) {
		// 确保在删除文本节点前先完成全部属性解析，避免后续 rebuild_raw_attrs 丢失尚未延迟解析的属性（issue 274）
		self.ensure_all_attrs();
		let mut out = Vec::with_capacity(self.children.len());
		for mut child in self.children.drain(..) {
			match &mut child {
				Node::Text(t) => {
					let mut t2 = t.clone();
					if !t2.is_whitespace() {
						let new_raw = {
							let _ = t2.trimmed_raw_text();
							t2.trimmed_raw_text().to_string()
						};
						t2.set_raw(new_raw);
						out.push(Node::Text(t2));
					}
				}
				Node::Element(e) => {
					let mut ec = e.clone();
					ec.remove_whitespace();
					out.push(Node::Element(ec));
				}
				Node::Comment(_) => {}
			}
		}
		self.children = out;
		self.rebuild_raw_attrs();
	}

	/// 模拟 JS HTMLElement.trimRight(pattern): 从右侧开始找到第一个匹配 TextNode 截断后续节点。
	pub fn trim_right(&mut self, pattern: &Regex) {
		let mut i = 0usize;
		while i < self.children.len() {
			match &mut self.children[i] {
				Node::Element(e) => {
					let mut ec = e.clone();
					ec.trim_right(pattern);
					self.children[i] = Node::Element(ec);
				}
				Node::Text(t) => {
					if let Some(mat) = pattern.find(&t.raw) {
						let new_raw = t.raw[..mat.start()].to_string();
						let mut nt = t.clone();
						nt.set_raw(new_raw);
						self.children[i] = Node::Text(nt);
						self.children.truncate(i + 1); // 截断后续
						return;
					}
				}
				Node::Comment(_) => {}
			}
			i += 1;
		}
	}

	/// 输出结构字符串（对应 JS structure 属性）。
	pub fn structure(&self) -> String {
		let mut res = Vec::new();
		fn dfs(cur: &HTMLElement, indent: usize, out: &mut Vec<String>) {
			if cur.is_root() {
				for child in &cur.children {
					if let Node::Element(e) = child {
						dfs(e, 0, out);
					}
				}
				return;
			}
			let mut line = String::new();
			line.push_str(&"  ".repeat(indent));
			line.push_str(cur.name());
			if !cur.id.is_empty() {
				line.push('#');
				line.push_str(&cur.id);
			}
			if let Some(cls) = cur.get_attr("class") {
				if !cls.is_empty() {
					// 去重，保持出现顺序
					let mut seen = std::collections::HashSet::new();
					for c in cls.split_whitespace() {
						if seen.insert(c) {
							line.push('.');
							line.push_str(c);
						}
					}
				}
			}
			out.push(line);
			for child in &cur.children {
				match child {
					Node::Element(e) => dfs(e, indent + 1, out),
					Node::Text(t) => {
						if !t.is_whitespace() {
							out.push(format!("{}#text", "  ".repeat(indent + 1)));
						}
					}
					Node::Comment(_) => {}
				}
			}
		}
		dfs(self, 0, &mut res);
		res.join("\n")
	}
	pub fn get_elements_by_tag_name<'a>(&'a self, tag: &str) -> Vec<&'a HTMLElement> {
		let tgt = tag.to_lowercase();
		let mut acc = Vec::new();
		fn walk<'b>(cur: &'b HTMLElement, tgt: &str, acc: &mut Vec<&'b HTMLElement>) {
			for c in &cur.children {
				if let Node::Element(e) = c {
					let inner = &**e;
					if tgt == "*" || inner.name().eq_ignore_ascii_case(tgt) {
						acc.push(inner);
					}
					walk(inner, tgt, acc);
				}
			}
		}
		walk(self, &tgt, &mut acc);
		acc
	}
	pub fn get_element_by_id<'a>(&'a self, id: &str) -> Option<&'a HTMLElement> {
		fn walk<'b>(cur: &'b HTMLElement, id: &str) -> Option<&'b HTMLElement> {
			for c in &cur.children {
				if let Node::Element(e) = c {
					let inner = &**e;
					if inner.get_attr("id") == Some(id) {
						return Some(inner);
					}
					if let Some(f) = walk(inner, id) {
						return Some(f);
					}
				}
			}
			None
		}
		walk(self, id)
	}
	pub fn get_element_by_id_mut<'a>(&'a mut self, id: &str) -> Option<&'a mut HTMLElement> {
		fn walk<'b>(cur: &'b mut HTMLElement, id: &str) -> Option<&'b mut HTMLElement> {
			for c in cur.children.iter_mut() {
				if let Node::Element(e) = c {
					// 优先使用缓存的 id 字段避免触发属性延迟解析
					if e.id == id || e.get_attr("id") == Some(id) {
						return Some(e);
					}
					if let Some(found) = walk(e, id) {
						return Some(found);
					}
				}
			}
			None
		}
		walk(self, id)
	}
	pub fn clone_node(&self) -> HTMLElement {
		fn clone_rec(el: &HTMLElement) -> Box<HTMLElement> {
			let mut new = Box::new(HTMLElement {
				tag_name: el.tag_name.clone(),
				raw_attrs: el.raw_attrs.clone(),
				attrs: el.attrs.clone(),
				children: Vec::new(),
				parent: None,
				is_void: el.is_void,
				void_add_slash: el.void_add_slash,
				cache_raw_map: None,
				cache_lower_decoded: None,

				id: el.id.clone(),
				class_cache: el.class_cache.clone(),
				range: None,
				attrs_complete: el.attrs_complete,
				attrs_modified: el.attrs_modified,
				parse_comment: el.parse_comment,
				parse_lowercase: el.parse_lowercase,
			});
			for c in &el.children {
				match c {
					Node::Element(e) => new.children.push(Node::Element(clone_rec(e))),
					Node::Text(t) => new.children.push(Node::Text(t.clone())),
					Node::Comment(cm) => new.children.push(Node::Comment(cm.clone())),
				};
			}
			new
		}
		*clone_rec(self)
	}
	/// 浅拷贝（不包含子节点）
	pub fn clone_shallow(&self) -> HTMLElement {
		HTMLElement {
			tag_name: self.tag_name.clone(),
			raw_attrs: self.raw_attrs.clone(),
			attrs: self.attrs.clone(),
			children: Vec::new(),
			parent: None,
			is_void: self.is_void,
			void_add_slash: self.void_add_slash,
			cache_raw_map: None,
			cache_lower_decoded: None,

			id: self.id.clone(),
			class_cache: self.class_cache.clone(),
			range: None,
			attrs_complete: self.attrs_complete,
			attrs_modified: self.attrs_modified,
			parse_comment: self.parse_comment,
			parse_lowercase: self.parse_lowercase,
		}
	}
	pub fn set_range_start(&mut self, start: usize) {
		match self.range {
			Some((_, e)) => self.range = Some((start, e)),
			None => self.range = Some((start, start)),
		}
	}
	pub fn set_range_end(&mut self, end: usize) {
		match self.range {
			Some((s, _)) => self.range = Some((s, end)),
			None => self.range = Some((end, end)),
		}
	}
	pub fn range(&self) -> Option<(usize, usize)> {
		self.range
	}

	/// 批量处理多个元素的属性解析 (启用parallel特性时使用rayon)
	/// 注意：由于线程安全约束，暂时使用串行处理
	#[cfg(feature = "parallel")]
	pub fn batch_ensure_attributes_safe(elements: &mut [HTMLElement]) {
		// 暂时使用串行处理以避免线程安全问题
		for el in elements.iter_mut() {
			el.ensure_all_attrs();
		}
	}

	/// 并行处理文本节点（线程安全版本）
	#[cfg(feature = "parallel")]
	pub fn process_text_nodes_parallel(text_nodes: &mut [crate::dom::text::TextNode]) {
		const PARALLEL_THRESHOLD: usize = 20;

		if text_nodes.len() >= PARALLEL_THRESHOLD {
			text_nodes.par_iter_mut().for_each(|node| {
				// 只处理不涉及DOM结构修改的操作
				let _ = node.is_whitespace();
				let _ = node.trimmed_raw_text();
			});
		}
	}
}

impl fmt::Display for HTMLElement {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.outer_html())
	}
}
