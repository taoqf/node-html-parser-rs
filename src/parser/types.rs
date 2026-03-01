//! 解析器相关的数据类型定义

use crate::dom::void_tag::VoidTagOptions;
use std::collections::HashMap;

/// 🔥 零拷贝版本 - 避免字符串分配，使用生命周期借用
#[derive(Debug, Clone)]
pub struct ZeroCopyTagMatch<'a> {
	pub start: usize,
	pub end: usize,
	pub is_comment: bool,
	pub is_closing: bool,
	pub tag_name: &'a str,
	pub attrs: &'a str,
	pub self_closing: bool,
}

#[derive(Debug, Clone)]
pub struct Options {
	pub lower_case_tag_name: bool,
	pub comment: bool,
	/// Corresponds to js option fixNestedATags
	pub fix_nested_a_tags: bool,
	/// Parse not-closed tags (do not attempt JS style repair) -> corresponds to parseNoneClosedTags
	pub parse_none_closed_tags: bool,
	/// When true, preserve tag nesting as-is and skip JS-style auto-closing repairs
	/// (corresponds to JS option `preserveTagNesting`).
	pub preserve_tag_nesting: bool,
	pub block_text_elements: HashMap<String, bool>, // tag -> ignore inner html when true
	/// When true, even if block_text_elements requests extraction for script/style, we suppress
	/// creating the inner raw Text node (used by tests expecting empty script/style by default).
	pub suppress_script_style_text: bool,
	pub void_tag: VoidTagOptions,
}

impl Default for Options {
	fn default() -> Self {
		let mut block = HashMap::new();
		// 默认：script/style/noscript/pre 都作为 block 文本元素，捕获其原始文本（不解析内部标签）
		block.insert("script".into(), true);
		block.insert("style".into(), true);
		block.insert("noscript".into(), true);
		block.insert("pre".into(), true);
		Self {
			lower_case_tag_name: false,
			comment: false,
			fix_nested_a_tags: false,
			parse_none_closed_tags: false,
			preserve_tag_nesting: false,
			block_text_elements: block,
			suppress_script_style_text: false,
			void_tag: Default::default(),
		}
	}
}

#[derive(Clone)]
pub(crate) struct StackEntry {
	pub elem: Box<crate::dom::element::HTMLElement>,
}
