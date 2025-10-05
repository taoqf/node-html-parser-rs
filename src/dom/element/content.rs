use super::main::HTMLElement;
use crate::dom::node::{CowStr, Node, NodeOrStr};
use crate::dom::text::TextNode;

impl HTMLElement {
	// ---------------- Mutation APIs (部分 JS parity) ----------------
	/// 解析片段为节点列表（不包裹额外 root），使用默认 Options
	/// insertAdjacentHTML(position, html)
	/// 支持: beforebegin | afterbegin | beforeend | afterend
	/// beforebegin/afterend 会插入到父节点 children 中（若无父则忽略）
	pub fn insert_adjacent_html(&mut self, position: &str, html: &str) -> Result<(), String> {
		let pos = position.to_lowercase();
		match pos.as_str() {
			"afterbegin" => self.insert_children_at(0, html),
			"beforeend" => {
				let len = self.children.len();
				self.insert_children_at(len, html);
			}
			"beforebegin" => self.insert_as_sibling(html, false),
			"afterend" => self.insert_as_sibling(html, true),
			_ => {
				return Err(format!(
					"The value provided ('{}') is not one of 'beforebegin', 'afterbegin', 'beforeend', or 'afterend'",
					position
				));
			}
		}
		Ok(())
	}

	/// replaceWith(html_fragment) 用片段替换自身
	pub fn replace_with(&mut self, html_fragment: &str) {
		self.replace_with_items(&[NodeOrStr::Str(CowStr(html_fragment))]);
	}
	pub fn replace_with_items(&mut self, items: &[NodeOrStr]) {
		let parent_ptr = match self.parent {
			Some(p) => p,
			None => return,
		};
		unsafe {
			let parent = &mut *parent_ptr;
			let idx = match self.index_in_parent() {
				Some(i) => i,
				None => return,
			};
			let mut nodes = collect_items(items);
			for n in nodes.iter_mut() {
				if let Node::Element(e) = n {
					e.parent = Some(parent_ptr);
				}
			}
			// 删除原节点
			parent.children.remove(idx);
			for (i, n) in nodes.into_iter().enumerate() {
				parent.children.insert(idx + i, n);
			}
		}
	}

	pub fn text(&self) -> String {
		html_escape::decode_html_entities(&self.raw_text()).to_string()
	}
	/// 对应 JS innerText: 返回未解码聚合文本 (即 rawText)
	pub fn inner_text(&self) -> String {
		self.raw_text()
	}
	/// 对应 JS textContent getter: 返回解码后的文本
	pub fn text_content(&self) -> String {
		self.text()
	}
	/// 对应 JS textContent setter: 先对传入进行实体编码，再替换子节点为单一文本节点
	pub fn set_text_content(&mut self, val: &str) {
		// JS HTMLElement.textContent 在实现中： this.childNodes = [ new TextNode(val, this) ] 并不对 val 做实体编码
		self.children.clear();
		self.children
			.push(Node::Text(TextNode::new(val.to_string())));
	}
	pub fn set_content_str(&mut self, content: &str, comment_override: Option<bool>) {
		let allow_comment = comment_override.unwrap_or(self.parse_comment);
		let (mut nodes, parsed_inner) = parse_fragment_with_opts(
			content,
			self.parse_comment,
			self.parse_lowercase,
			comment_override,
		);
		if !allow_comment {
			// 过滤掉注释节点
			nodes.retain(|n| !matches!(n, Node::Comment(_)));
		}
		// JS: 若解析后 childNodes 为空，则回退为 TextNode(r.innerHTML) 而不是原始 content
		let fallback = if nodes.is_empty() {
			parsed_inner.as_str()
		} else {
			content
		};
		self.replace_children_with(nodes, fallback);
	}

	fn replace_children_with(&mut self, mut nodes: Vec<Node>, raw_fallback: &str) {
		if nodes.is_empty() {
			use crate::dom::text::TextNode;
			nodes.push(Node::Text(TextNode::new(raw_fallback.to_string())));
		}
		let self_ptr: *mut HTMLElement = self as *mut HTMLElement;
		for n in nodes.iter_mut() {
			if let Node::Element(e) = n {
				e.parent = Some(self_ptr);
			}
		}
		self.children.clear();
		self.children.extend(nodes);
	}

	/// JS 兼容别名：set_content(字符串) -> set_content_str，无注释覆盖。
	pub fn set_content(&mut self, content: &str) {
		self.set_content_str(content, None);
	}
	pub fn set_content_nodes(&mut self, mut nodes: Vec<Node>) {
		let self_ptr: *mut HTMLElement = self as *mut HTMLElement;
		for n in nodes.iter_mut() {
			if let Node::Element(e) = n {
				e.parent = Some(self_ptr);
			}
		}
		self.children.clear();
		self.children.extend(nodes);
	}
}

fn parse_fragment_with_opts(
	html: &str,
	base_comment: bool,
	lower: bool,
	override_comment: Option<bool>,
) -> (Vec<Node>, String) {
	use crate::parser::{parse_with_options, Options};
	let mut opts = Options::default();
	// JS set_content 行为：若传入 override 则使用其值；否则继承 base_comment。
	opts.comment = override_comment.unwrap_or(base_comment);
	opts.lower_case_tag_name = lower;
	let mut root = parse_with_options(html, &opts);
	let fallback_inner = root.inner_html();
	(root.children.drain(..).collect(), fallback_inner)
}
pub(super) fn parse_fragment(html: &str) -> Vec<Node> {
	use crate::parser::{parse_with_options, Options};
	let mut opts = Options::default();
	if html.contains("<!--") {
		opts.comment = true;
	}
	let mut root = parse_with_options(html, &opts);
	root.children.drain(..).collect()
}

pub(super) fn collect_items(items: &[NodeOrStr]) -> Vec<Node> {
	let mut out = Vec::new();
	for it in items {
		match it {
			NodeOrStr::Str(s) => {
				let mut frag = parse_fragment(&s.0);
				out.extend(frag.drain(..));
			}
			NodeOrStr::Existing(n) => out.push(n.clone()), // 暂用 clone 语义
		}
	}
	out
}
