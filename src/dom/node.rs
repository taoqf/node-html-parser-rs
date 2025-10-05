use crate::dom::{comment::CommentNode, element::HTMLElement, node_type::NodeType, text::TextNode};

#[derive(Debug, Clone)]
pub enum Node {
	Element(Box<HTMLElement>),
	Text(TextNode),
	Comment(CommentNode),
}

#[derive(Debug, Clone)]
pub enum NodeOrStr<'a> {
	Str(CowStr<'a>),
	Existing(Node),
}

#[derive(Debug, Clone)]
pub struct CowStr<'a>(pub &'a str);
impl<'a> From<&'a str> for CowStr<'a> {
	fn from(s: &'a str) -> Self {
		CowStr(s)
	}
}
impl<'a> std::ops::Deref for CowStr<'a> {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		self.0
	}
}

impl Node {
	pub fn node_type(&self) -> NodeType {
		match self {
			Node::Element(_) => NodeType::Element,
			Node::Text(_) => NodeType::Text,
			Node::Comment(_) => NodeType::Comment,
		}
	}
	pub fn as_element(&self) -> Option<&HTMLElement> {
		if let Node::Element(e) = self {
			Some(e)
		} else {
			None
		}
	}
	pub fn as_element_mut(&mut self) -> Option<&mut HTMLElement> {
		if let Node::Element(e) = self {
			Some(e)
		} else {
			None
		}
	}
	pub fn raw_text(&self) -> String {
		match self {
			Node::Element(e) => e.raw_text(),
			Node::Text(t) => t.raw.clone(),
			Node::Comment(c) => c.text.clone(),
		}
	}
	pub fn text(&self) -> String {
		match self {
			Node::Element(e) => e.text(),
			Node::Text(t) => html_escape::decode_html_entities(&t.raw).to_string(),
			Node::Comment(c) => c.text.clone(),
		}
	}
	pub fn to_html(&self) -> String {
		match self {
			Node::Element(e) => e.outer_html(),
			Node::Text(t) => t.raw.clone(),
			Node::Comment(c) => format!("<!--{}-->", c.text),
		}
	}
}
