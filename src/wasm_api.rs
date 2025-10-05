#![allow(clippy::missing_safety_doc)]
// wasm 导出封装：对内保持现有 Rust 结构，对外提供稳定 JS API
// 目标：暴露 parse / parse_with_options / valid 以及 HTMLElement 的核心读写接口
// 策略：
// 1. 不直接暴露内部 HTMLElement / Node 枚举，避免递归/指针生命周期复杂性
// 2. 提供 JsElement 句柄（持有 Box<HTMLElement> 或 Rc<RefCell<HTMLElement>>）
// 3. 暂不暴露实时 DOM 变更后结构之间的引用（简化）；后续可加全局 Arena

use wasm_bindgen::prelude::*;

use crate::dom::comment::CommentNode;
use crate::dom::element::main::HTMLElement;
use crate::dom::node::Node;
use crate::dom::text::TextNode;
use crate::parser::{parse, parse_with_options, valid, Options};
use regex::Regex;

#[wasm_bindgen]
pub struct JsParseOptions {
	lower_case_tag_name: bool,
	comment: bool,
	fix_nested_a_tags: bool,
	parse_none_closed_tags: bool,
}

#[wasm_bindgen]
impl JsParseOptions {
	#[wasm_bindgen(constructor)]
	pub fn new() -> JsParseOptions {
		JsParseOptions {
			lower_case_tag_name: false,
			comment: false,
			fix_nested_a_tags: false,
			parse_none_closed_tags: false,
		}
	}
	#[wasm_bindgen(getter)]
	pub fn lower_case_tag_name(&self) -> bool {
		self.lower_case_tag_name
	}
	#[wasm_bindgen(setter)]
	pub fn set_lower_case_tag_name(&mut self, v: bool) {
		self.lower_case_tag_name = v;
	}
	#[wasm_bindgen(getter)]
	pub fn comment(&self) -> bool {
		self.comment
	}
	#[wasm_bindgen(setter)]
	pub fn set_comment(&mut self, v: bool) {
		self.comment = v;
	}
	#[wasm_bindgen(getter)]
	pub fn fix_nested_a_tags(&self) -> bool {
		self.fix_nested_a_tags
	}
	#[wasm_bindgen(setter)]
	pub fn set_fix_nested_a_tags(&mut self, v: bool) {
		self.fix_nested_a_tags = v;
	}
	#[wasm_bindgen(getter)]
	pub fn parse_none_closed_tags(&self) -> bool {
		self.parse_none_closed_tags
	}
	#[wasm_bindgen(setter)]
	pub fn set_parse_none_closed_tags(&mut self, v: bool) {
		self.parse_none_closed_tags = v;
	}
}

impl JsParseOptions {
	fn to_native(&self) -> Options {
		let mut o = Options::default();
		o.lower_case_tag_name = self.lower_case_tag_name;
		o.comment = self.comment;
		o.fix_nested_a_tags = self.fix_nested_a_tags;
		o.parse_none_closed_tags = self.parse_none_closed_tags;
		o
	}
}

#[wasm_bindgen]
pub struct JsElement {
	inner: Box<HTMLElement>,
}

impl JsElement {
	fn from_box(b: Box<HTMLElement>) -> JsElement {
		JsElement { inner: b }
	}
}

#[wasm_bindgen]
impl JsElement {
	#[wasm_bindgen(getter)]
	pub fn tag_name(&self) -> String {
		self.inner.name().to_ascii_uppercase()
	}

	#[wasm_bindgen(getter)]
	pub fn id(&self) -> String {
		self.inner.id.clone()
	}

	#[wasm_bindgen(js_name = getAttribute)]
	pub fn get_attribute(&mut self, name: &str) -> Option<String> {
		self.inner.get_attribute(name)
	}

	#[wasm_bindgen(js_name = setAttribute)]
	pub fn set_attribute(&mut self, name: &str, value: &str) {
		self.inner.set_attribute(name, value);
	}

	#[wasm_bindgen(js_name = hasAttribute)]
	pub fn has_attribute(&mut self, name: &str) -> bool {
		self.inner.has_attribute(name)
	}

	#[wasm_bindgen(js_name = removeAttribute)]
	pub fn remove_attribute(&mut self, name: &str) {
		self.inner.remove_attribute(name);
	}

	#[wasm_bindgen(getter, js_name = innerHTML)]
	pub fn inner_html(&self) -> String {
		self.inner.inner_html()
	}
	#[wasm_bindgen(setter, js_name = innerHTML)]
	pub fn set_inner_html(&mut self, html: &str) {
		self.inner.set_inner_html(html);
	}

	#[wasm_bindgen(getter, js_name = outerHTML)]
	pub fn outer_html(&self) -> String {
		self.inner.outer_html()
	}

	#[wasm_bindgen(getter, js_name = textContent)]
	pub fn text_content(&self) -> String {
		self.inner.text_content()
	}
	#[wasm_bindgen(setter, js_name = textContent)]
	pub fn set_text_content(&mut self, v: &str) {
		self.inner.set_text_content(v);
	}

	#[wasm_bindgen(js_name = querySelector)]
	pub fn query_selector(&self, selector: &str) -> Option<JsElement> {
		let root = &self.inner;
		let found = root.query_selector(selector)?;
		// clone subtree for isolation
		Some(JsElement::from_box(Box::new(found.clone_node())))
	}
	#[wasm_bindgen(js_name = querySelectorAll)]
	pub fn query_selector_all(&self, selector: &str) -> Vec<JsElement> {
		let root = &self.inner;
		root.query_selector_all(selector)
			.into_iter()
			.map(|e| JsElement::from_box(Box::new(e.clone_node())))
			.collect()
	}

	#[wasm_bindgen(js_name = childNodes)]
	pub fn child_nodes(&self) -> Vec<JsNode> {
		self.inner
			.children
			.iter()
			.map(|c| JsNode::from_node(c))
			.collect()
	}

	#[wasm_bindgen(js_name = firstChild)]
	pub fn first_child(&self) -> Option<JsNode> {
		self.inner.children.first().map(|n| JsNode::from_node(n))
	}
	#[wasm_bindgen(js_name = lastChild)]
	pub fn last_child(&self) -> Option<JsNode> {
		self.inner.children.last().map(|n| JsNode::from_node(n))
	}

	// ================= Additional DOM-like APIs =================
	#[wasm_bindgen(js_name = structure)]
	pub fn structure_string(&self) -> String {
		self.inner.structure()
	}

	#[wasm_bindgen(js_name = getElementsByTagName)]
	pub fn get_elements_by_tag_name_export(&self, tag: &str) -> Vec<JsElement> {
		self.inner
			.get_elements_by_tag_name(tag)
			.into_iter()
			.map(|e| JsElement::from_box(Box::new(e.clone_node())))
			.collect()
	}

	#[wasm_bindgen(js_name = getElementById)]
	pub fn get_element_by_id_export(&self, id: &str) -> Option<JsElement> {
		self.inner
			.get_element_by_id(id)
			.map(|e| JsElement::from_box(Box::new(e.clone_node())))
	}

	#[wasm_bindgen(js_name = matches)]
	pub fn matches_selector(&self, selector: &str) -> bool {
		self.inner.matches(selector)
	}

	#[wasm_bindgen(js_name = closest)]
	pub fn closest_selector(&self, selector: &str) -> Option<JsElement> {
		self.inner
			.closest(selector)
			.map(|e| JsElement::from_box(Box::new(e.clone_node())))
	}

	#[wasm_bindgen(js_name = clone)]
	pub fn clone_deep(&self) -> JsElement {
		JsElement::from_box(Box::new(self.inner.clone_node()))
	}

	#[wasm_bindgen(js_name = cloneShallow)]
	pub fn clone_shallow_export(&self) -> JsElement {
		JsElement::from_box(Box::new(self.inner.clone_shallow()))
	}

	#[wasm_bindgen(js_name = setContent)]
	pub fn set_content_export(&mut self, html: &str) {
		self.inner.set_content(html);
	}

	#[wasm_bindgen(js_name = replaceWith)]
	pub fn replace_with_export(&mut self, html_fragment: &str) {
		self.inner.replace_with(html_fragment);
	}

	#[wasm_bindgen(js_name = insertAdjacentHTML)]
	pub fn insert_adjacent_html_export(&mut self, position: &str, html: &str) -> bool {
		self.inner.insert_adjacent_html(position, html).is_ok()
	}

	#[wasm_bindgen(js_name = removeWhitespace)]
	pub fn remove_whitespace_export(&mut self) {
		self.inner.remove_whitespace();
	}

	#[wasm_bindgen(js_name = trimRight)]
	pub fn trim_right_export(&mut self, pattern: &str) -> bool {
		match Regex::new(pattern) {
			Ok(re) => {
				self.inner.trim_right(&re);
				true
			}
			Err(_) => false,
		}
	}

	#[wasm_bindgen(js_name = attributesMap)]
	pub fn attributes_map(&mut self) -> js_sys::Object {
		let obj = js_sys::Object::new();
		for (k, v) in self.inner.attributes() {
			let _ = js_sys::Reflect::set(&obj, &JsValue::from_str(&k), &JsValue::from_str(&v));
		}
		obj
	}

	#[wasm_bindgen(js_name = children)]
	pub fn children_elements(&self) -> Vec<JsElement> {
		self.inner
			.children
			.iter()
			.filter_map(|n| {
				if let Node::Element(ref e) = n {
					Some(JsElement::from_box(Box::new(e.clone_node())))
				} else {
					None
				}
			})
			.collect()
	}

	#[wasm_bindgen(js_name = childElementCount)]
	pub fn child_element_count(&self) -> u32 {
		self.inner
			.children
			.iter()
			.filter(|n| matches!(n, Node::Element(_)))
			.count() as u32
	}

	#[wasm_bindgen(js_name = firstElementChild)]
	pub fn first_element_child(&self) -> Option<JsElement> {
		self.inner.children.iter().find_map(|n| {
			if let Node::Element(ref e) = n {
				Some(JsElement::from_box(Box::new(e.clone_node())))
			} else {
				None
			}
		})
	}

	#[wasm_bindgen(js_name = lastElementChild)]
	pub fn last_element_child(&self) -> Option<JsElement> {
		self.inner.children.iter().rev().find_map(|n| {
			if let Node::Element(ref e) = n {
				Some(JsElement::from_box(Box::new(e.clone_node())))
			} else {
				None
			}
		})
	}

	#[wasm_bindgen(js_name = rawText)]
	pub fn raw_text_export(&self) -> String {
		self.inner.raw_text()
	}

	#[wasm_bindgen(js_name = text)]
	pub fn text_export(&self) -> String {
		self.inner.text()
	}

	#[wasm_bindgen(js_name = toString)]
	pub fn to_string_export(&self) -> String {
		self.inner.outer_html()
	}

	#[wasm_bindgen(js_name = appendChild)]
	pub fn append_child(&mut self, html_fragment: &str) -> bool {
		self.inner
			.insert_adjacent_html("beforeend", html_fragment)
			.is_ok()
	}

	#[wasm_bindgen(js_name = appendChildElement)]
	pub fn append_child_element(&mut self, el: &JsElement) -> bool {
		let mut cloned = Box::new(el.inner.clone_node());
		let self_ptr: *mut HTMLElement = &mut *self.inner;
		cloned.parent = Some(self_ptr);
		self.inner.children.push(Node::Element(cloned));
		true
	}

	#[wasm_bindgen(js_name = appendChildText)]
	pub fn append_child_text(&mut self, txt: &JsTextNode) -> bool {
		self.inner.children.push(Node::Text(txt.inner.clone()));
		true
	}
}

// ---------------- Generic Node Wrapper ----------------
// ---- Unified Node wrapper & specific node types ----
#[wasm_bindgen]
pub enum JsNodeType {
	Element = 1,
	Text = 3,
	Comment = 8,
}

#[wasm_bindgen]
pub struct JsNode {
	inner: Box<Node>,
}

impl JsNode {
	fn from_node(n: &Node) -> JsNode {
		JsNode {
			inner: Box::new(n.clone()),
		}
	}
}

#[wasm_bindgen]
impl JsNode {
	#[wasm_bindgen(js_name = nodeType)]
	pub fn node_type(&self) -> JsNodeType {
		match *self.inner {
			Node::Element(_) => JsNodeType::Element,
			Node::Text(_) => JsNodeType::Text,
			Node::Comment(_) => JsNodeType::Comment,
		}
	}
	#[wasm_bindgen(js_name = asElement)]
	pub fn as_element(&self) -> Option<JsElement> {
		if let Node::Element(ref e) = *self.inner {
			Some(JsElement::from_box(Box::new(e.clone_node())))
		} else {
			None
		}
	}
	#[wasm_bindgen(js_name = asText)]
	pub fn as_text(&self) -> Option<JsTextNode> {
		if let Node::Text(ref t) = *self.inner {
			Some(JsTextNode { inner: t.clone() })
		} else {
			None
		}
	}
	#[wasm_bindgen(js_name = asComment)]
	pub fn as_comment(&self) -> Option<JsCommentNode> {
		if let Node::Comment(ref c) = *self.inner {
			Some(JsCommentNode { inner: c.clone() })
		} else {
			None
		}
	}
	#[wasm_bindgen(js_name = isWhitespace)]
	pub fn is_whitespace(&self) -> bool {
		matches!(*self.inner, Node::Text(ref t) if t.is_whitespace())
	}
	#[wasm_bindgen(getter, js_name = textContent)]
	pub fn text_content(&self) -> String {
		self.inner.text()
	}
	#[wasm_bindgen(js_name = rawText)]
	pub fn raw_text(&self) -> String {
		self.inner.raw_text()
	}
	#[wasm_bindgen(js_name = toHTML)]
	pub fn to_html(&self) -> String {
		self.inner.to_html()
	}
	#[wasm_bindgen(js_name = toString)]
	pub fn node_to_string_export(&self) -> String {
		self.inner.to_html()
	}
	#[wasm_bindgen(js_name = range)]
	pub fn range(&self) -> Option<js_sys::Array> {
		match *self.inner {
			Node::Element(ref e) => e.range(),
			Node::Text(ref t) => t.range(),
			Node::Comment(ref c) => c.range(),
		}
		.map(|(s, e)| {
			let a = js_sys::Array::new();
			a.push(&JsValue::from_f64(s as f64));
			a.push(&JsValue::from_f64(e as f64));
			a
		})
	}
}

#[wasm_bindgen]
pub struct JsTextNode {
	inner: TextNode,
}
#[wasm_bindgen]
impl JsTextNode {
	#[wasm_bindgen(getter, js_name = rawText)]
	pub fn raw_text(&self) -> String {
		self.inner.raw_text().to_string()
	}
	#[wasm_bindgen(getter, js_name = textContent)]
	pub fn text_content(&self) -> String {
		self.inner.text()
	}
	#[wasm_bindgen(js_name = isWhitespace)]
	pub fn is_whitespace(&self) -> bool {
		self.inner.is_whitespace()
	}
	#[wasm_bindgen(js_name = toHTML)]
	pub fn to_html(&self) -> String {
		self.inner.raw_text().to_string()
	}
	#[wasm_bindgen(js_name = toString)]
	pub fn textnode_to_string_export(&self) -> String {
		self.inner.raw_text().to_string()
	}
	#[wasm_bindgen(js_name = range)]
	pub fn range(&self) -> Option<js_sys::Array> {
		self.inner.range().map(|(s, e)| {
			let a = js_sys::Array::new();
			a.push(&JsValue::from_f64(s as f64));
			a.push(&JsValue::from_f64(e as f64));
			a
		})
	}
}

#[wasm_bindgen]
pub struct JsCommentNode {
	inner: CommentNode,
}
#[wasm_bindgen]
impl JsCommentNode {
	#[wasm_bindgen(getter, js_name = text)]
	pub fn text(&self) -> String {
		self.inner.text.clone()
	}
	#[wasm_bindgen(js_name = toHTML)]
	pub fn to_html(&self) -> String {
		format!("<!--{}-->", self.inner.text)
	}
	#[wasm_bindgen(js_name = toString)]
	pub fn comment_to_string_export(&self) -> String {
		format!("<!--{}-->", self.inner.text)
	}
	#[wasm_bindgen(js_name = range)]
	pub fn range(&self) -> Option<js_sys::Array> {
		self.inner.range().map(|(s, e)| {
			let a = js_sys::Array::new();
			a.push(&JsValue::from_f64(s as f64));
			a.push(&JsValue::from_f64(e as f64));
			a
		})
	}
}

#[wasm_bindgen]
pub fn parse_html(input: &str) -> JsElement {
	JsElement::from_box(parse(input))
}

#[wasm_bindgen]
pub fn parse_html_with_options(input: &str, opts: &JsParseOptions) -> JsElement {
	let native = opts.to_native();
	JsElement::from_box(parse_with_options(input, &native))
}

#[wasm_bindgen]
pub fn is_valid_html(input: &str) -> bool {
	valid(input, &Options::default())
}

#[wasm_bindgen]
pub fn version() -> String {
	env!("CARGO_PKG_VERSION").to_string()
}
