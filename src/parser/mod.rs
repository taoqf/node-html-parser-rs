//! Minimal Rust translation of core logic from js/node-html-parser.
//! Provides a `parse` function returning the root Element whose
//! children correspond to the original HTML fragment.
use crate::dom::comment::CommentNode;
use crate::dom::element::HTMLElement;
use crate::dom::node::Node;
use crate::dom::text::TextNode;
use crate::dom::void_tag::{VoidTag, VoidTagOptions};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Options {
	pub lower_case_tag_name: bool,
	pub comment: bool,
	/// Corresponds to js option fixNestedATags
	pub fix_nested_a_tags: bool,
	/// Parse not-closed tags (do not attempt JS style repair) -> corresponds to parseNoneClosedTags
	pub parse_none_closed_tags: bool,
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
			block_text_elements: block,
			suppress_script_style_text: false,
			void_tag: Default::default(),
		}
	}
}

pub fn parse(input: &str) -> Box<HTMLElement> {
	parse_with_options(input, &Options::default())
}

pub fn parse_with_options(input: &str, opts: &Options) -> Box<HTMLElement> {
	// 改进：
	// 1) 支持引号内含 '>'（通过 attrs 子模式保证只有引号外的 '>' 终止标签）
	// 2) 扩展 tagName 的 Unicode 范围以对齐 JS 版本 kMarkupPattern，使自定义标签（含中文、阿拉伯等 BMP 范围字符）通过。
	//    JS 的模式包含大量 Unicode 范围及高位平面；Rust regex 不支持 >BMP 直接类，这里截取到 BMP（满足当前测试需求）。
	let tag_re = Regex::new(r#"<!--(?s:.*?)-->|<(\/)?([A-Za-z][A-Za-z0-9._:@\-\p{L}\p{M}]*)(?P<attrs>(?:[^>"']|"[^"]*"|'[^']*')*)(/?)>"#).unwrap();
	let attr_key_re = Regex::new(
		r#"([a-zA-Z()\[\]#@$.?:][a-zA-Z0-9-._:()\[\]#]*)(?:\s*=\s*((?:'[^']*')|(?:"[^"]*")|[^\s>]+))?"#,
	)
	.unwrap();
	let void_tag = VoidTag::new(&opts.void_tag);

	// Elements closed by opening of specific following tags (subset of JS map)
	let mut closed_by_open: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
	// helper: insert mapping for both lower & upper parent names (mirror JS maps having LI & li etc.)
	macro_rules! ins_open {
		($parent:expr, [ $($v:expr),* ]) => {{
			closed_by_open.insert($parent, vec![$($v),*]);
			let up = $parent.to_uppercase();
			if up != $parent { closed_by_open.insert(Box::leak(up.into_boxed_str()), vec![$($v),*]); }
		}};
	}
	ins_open!("li", ["li", "LI"]);
	ins_open!("p", ["p", "P", "div", "DIV"]);
	ins_open!("b", ["div", "DIV"]);
	ins_open!("td", ["td", "th", "TD", "TH"]);
	ins_open!("th", ["td", "th", "TD", "TH"]);
	ins_open!("h1", ["h1", "H1"]);
	ins_open!("h2", ["h2", "H2"]);
	ins_open!("h3", ["h3", "H3"]);
	ins_open!("h4", ["h4", "H4"]);
	ins_open!("h5", ["h5", "H5"]);
	ins_open!("h6", ["h6", "H6"]);

	let mut closed_by_close: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
	macro_rules! ins_close {
		($parent:expr, [ $($v:expr),* ]) => {{
			closed_by_close.insert($parent, vec![$($v),*]);
			let up = $parent.to_uppercase();
			if up != $parent { closed_by_close.insert(Box::leak(up.into_boxed_str()), vec![$($v),*]); }
		}};
	}
	ins_close!("li", ["ul", "UL", "ol", "OL"]);
	ins_close!("a", ["div", "DIV"]);
	ins_close!("b", ["div", "DIV"]);
	ins_close!("i", ["div", "DIV"]);
	ins_close!("p", ["div", "DIV"]);
	ins_close!("td", ["tr", "TR", "table", "TABLE"]);
	ins_close!("th", ["tr", "TR", "table", "TABLE"]);

	#[derive(Clone)]
	struct StackEntry {
		elem: Box<HTMLElement>,
	}
	let root = Box::new(HTMLElement::new(
		None,
		String::new(),
		Vec::new(),
		false,
		opts.void_tag.add_closing_slash,
	));
	let mut stack: Vec<StackEntry> = vec![StackEntry { elem: root }];
	// Safe mutable access to set options flags on root
	if let Some(first) = stack.last_mut() {
		first.elem.parse_comment = opts.comment;
		first.elem.parse_lowercase = opts.lower_case_tag_name;
	}
	let mut last_text_pos = 0usize;
	let mut no_nested_a_index: Option<usize> = None;
	// 记录在 block 文本处理阶段被直接消耗的关闭标签位置
	let mut skipped_closing_starts: std::collections::HashSet<usize> =
		std::collections::HashSet::new();

	let frameflag = "documentfragmentcontainer";
	let frame_prefix = format!("<{}>", frameflag);
	let frame_suffix = format!("</{}>", frameflag);
	let data = format!("{}{}{}", frame_prefix, input, frame_suffix);
	let frame_offset = frame_prefix.len();
	// 遍历标签匹配（正则已保证不会把引号内的 '>' 视作结束）
	for m in tag_re.captures_iter(&data) {
		let full = m.get(0).unwrap();
		// 若该匹配为已消耗的 blockTextElements 的关闭标签，直接跳过（模拟 JS 在 block 读取中提前消耗关闭标签）
		if skipped_closing_starts.contains(&full.start()) {
			continue;
		}
		let leading_slash = m.get(1).map(|c| c.as_str()).unwrap_or("");
		let mut tag_name = m.get(2).map(|c| c.as_str()).unwrap_or("").to_string();
		let is_comment = full.as_str().starts_with("<!--");
		let raw_attr_part = m.name("attrs").map(|c| c.as_str()).unwrap_or("");
		let closing_slash_cap = m.get(4).map(|c| c.as_str()).unwrap_or("");
		// 为了贴合 JS 行为：kMarkupPattern 中 attributes 与末尾的可选 "/" 分离；
		// 我们的正则会把末尾的自闭合斜杠吞进 attrs（因为 / 不被排除）。
		// 修正策略：从右向左扫描，若发现未在引号内的末尾 '/'，则把它视作自闭合标记并剥离。
		fn strip_trailing_self_close(s: &str) -> (String, bool) {
			let mut in_single = false;
			let mut in_double = false;
			// 反向扫描需要掌握每个字符的引号状态；简单做法：正向一次记录状态，然后再反向索引。
			let chars: Vec<char> = s.chars().collect();
			let mut quote_state: Vec<(bool, bool)> = Vec::with_capacity(chars.len());
			for &ch in &chars {
				match ch {
					'"' if !in_single => in_double = !in_double,
					'\'' if !in_double => in_single = !in_single,
					_ => {}
				}
				quote_state.push((in_single, in_double));
			}
			// 反向跳过空白
			let mut idx = chars.len();
			while idx > 0 && chars[idx - 1].is_whitespace() {
				idx -= 1;
			}
			if idx > 0 && chars[idx - 1] == '/' {
				let (s_in, d_in) = quote_state[idx - 1];
				if !s_in && !d_in {
					// 确认不在引号内
					// 再检查前一个非空白字符是否是 '='（防止把值里的孤立 / 当成标记，但 alt="Path/"/> 情况最后一个 / 属于值内部，后面还有 '"' 再有 '/' self-close）
					// 在 alt="Path/"/> 中 attr_part 末尾会是 alt="Path/"/ ：倒数第二个字符是 '"'
					// 此时 idx-2 是 '"', 再往前是 'h'. 可以安全剥离。
					let cleaned = chars[..idx - 1].iter().collect();
					return (cleaned, true);
				}
			}
			(s.to_string(), false)
		}
		let (attr_part, trailing_self_close) = strip_trailing_self_close(raw_attr_part);
		let match_start = full.start();
		if match_start > last_text_pos {
			let text = &data[last_text_pos..match_start];
			if !text.is_empty() {
				let top = stack.last_mut().unwrap();
				let start_src = last_text_pos.saturating_sub(frame_offset);
				let end_src = match_start.saturating_sub(frame_offset);
				top.elem.children.push(Node::Text(TextNode::with_range(
					text.to_string(),
					start_src,
					end_src,
				)));
			}
		}
		last_text_pos = full.end();
		if is_comment {
			if opts.comment {
				let inner = full
					.as_str()
					.trim_start_matches("<!--")
					.trim_end_matches("-->");
				let top = stack.last_mut().unwrap();
				// 记录注释在原输入中的范围（包含 <!-- --> 符号），与元素/文本一样使用 (start,end) 半开区间
				let start_src = match_start.saturating_sub(frame_offset);
				let end_src = full.end().saturating_sub(frame_offset);
				top.elem
					.children
					.push(Node::Comment(CommentNode::with_range(
						inner.to_string(),
						start_src,
						end_src,
					)));
			}
			continue;
		}
		if tag_name == frameflag {
			continue;
		}
		if opts.lower_case_tag_name {
			tag_name = tag_name.to_lowercase();
		}
		let mut self_closing = !closing_slash_cap.is_empty()
			|| trailing_self_close
			|| attr_part.trim_end().ends_with('/') // 保险：兼容旧逻辑（理论上已剥离）
			|| void_tag.is_void(&tag_name);

		if leading_slash.is_empty() {
			// opening tag
			// auto close logic (original behavior: just pop parent when needed)
			if let Some(parent) = stack.last() {
				if let Some(list) = closed_by_open.get(parent.elem.name()) {
					if list
						.iter()
						.any(|t| **t == tag_name || *t == tag_name.to_uppercase())
					{
						if stack.len() > 1 {
							let closed = stack.pop().unwrap();
							stack
								.last_mut()
								.unwrap()
								.elem
								.children
								.push(Node::Element(closed.elem));
						}
					}
				}
			}
			// fix nested A tags
			if opts.fix_nested_a_tags && (tag_name.eq("a") || tag_name.eq("A")) {
				if let Some(idx) = no_nested_a_index {
					while stack.len() > idx {
						let closed = stack.pop().unwrap();
						stack
							.last_mut()
							.unwrap()
							.elem
							.children
							.push(Node::Element(closed.elem));
					}
				}
				no_nested_a_index = Some(stack.len());
			}

			// 初始阶段仅收集 id 和 class，其余延迟解析（更贴近 JS base_parse 行为）
			let mut attrs: Vec<(String, String)> = Vec::new();
			let mut saw_other_attr = false;
			for cap in attr_key_re.captures_iter(&attr_part) {
				let k = cap.get(1).unwrap().as_str();
				let v_raw_opt = cap.get(2).or(cap.get(3)).or(cap.get(4));
				let raw_v = v_raw_opt.map(|v| v.as_str()).unwrap_or("");
				let unquoted = if raw_v.starts_with('"') || raw_v.starts_with('\'') {
					raw_v.trim_matches(['"', '\''])
				} else {
					raw_v
				};
				let lk = k.to_lowercase();
				if lk == "id" || lk == "class" {
					attrs.push((lk, html_escape::decode_html_entities(unquoted).to_string()));
				} else {
					saw_other_attr = true;
				}
			}
			let raw_attr_string = if attr_part.starts_with(' ') {
				attr_part[1..].to_string()
			} else {
				attr_part.to_string()
			};
			let mut elem = Box::new(HTMLElement::new(
				Some(tag_name.clone()),
				raw_attr_string,
				attrs,
				self_closing && void_tag.is_void(&tag_name),
				opts.void_tag.add_closing_slash,
			));
			// range 将在真正闭合或自闭合时由 set_range_start / set_range_end 赋值
			// 记录解析选项供后续 set_content 继承
			elem.parse_comment = opts.comment;
			elem.parse_lowercase = opts.lower_case_tag_name;
			if saw_other_attr {
				elem.attrs_complete = false;
			}
			let open_start = match_start.saturating_sub(frame_offset);
			let open_end = full.end().saturating_sub(frame_offset);
			// provisional start
			elem.set_range_start(open_start);
			if self_closing {
				// for self-closing tag finalize range
				elem.set_range_end(open_end);
			}
			// 解析阶段填充 id 字段，便于后续 structure / closest 等直接使用（与 JS keyAttrs.id 行为对齐）
			if let Some((_, v)) = elem.attrs.iter().find(|(k, _)| k == "id") {
				elem.id = v.clone();
			}

			// block text element handling: capture inner text verbatim until closing tag
			let lower_tag = tag_name.to_lowercase();
			if let Some(extract) = opts.block_text_elements.get(&lower_tag) {
				// 查找对应关闭标签（大小写不敏感，参考 JS 逻辑）
				let close_markup = format!("</{}>", tag_name);
				let search_slice = &data[last_text_pos..];
				let lower_search = search_slice.to_lowercase();
				let lower_close = close_markup.to_lowercase();
				if let Some(rel) = lower_search.find(&lower_close) {
					let close_start = last_text_pos + rel; // 关闭标签 '<' 起始位置
					let suppress = opts.suppress_script_style_text
						&& (lower_tag == "script" || lower_tag == "style");
					if *extract && !suppress {
						// true -> 提取文本（除非被全局抑制）
						let inner_text = &data[last_text_pos..close_start];
						if !inner_text.is_empty() {
							let inner_start = last_text_pos.saturating_sub(frame_offset);
							let inner_end = close_start.saturating_sub(frame_offset);
							elem.children.push(Node::Text(TextNode::with_range(
								inner_text.to_string(),
								inner_start,
								inner_end,
							)));
						}
					}
					// 无论是否提取文本，都跳过关闭标签匹配：记录其起始位置，供主循环跳过
					skipped_closing_starts.insert(close_start);
					last_text_pos = close_start + close_markup.len();
					let close_end_src =
						(close_start + close_markup.len()).saturating_sub(frame_offset);
					elem.set_range_end(close_end_src);
					self_closing = true; // 强制立即闭合
				} else {
					// 未找到关闭标签：与 JS 一致，标记 last_text_pos 到末尾避免后续文本节点重复
					last_text_pos = data.len() + 1;
				}
			}
			// 若因 block_textElements 强制 self_closing，将 elem.children 保持现状并不入栈

			if self_closing {
				let parent = stack.last_mut().unwrap();
				let parent_ptr: *mut HTMLElement = &mut *parent.elem;
				elem.parent = Some(parent_ptr);
				parent.elem.children.push(Node::Element(elem));
			} else {
				let parent_ptr: *mut HTMLElement = &mut *stack.last_mut().unwrap().elem;
				elem.parent = Some(parent_ptr);
				stack.push(StackEntry { elem });
			}
		} else {
			// closing tag
			// remove nested a index if closing A
			if no_nested_a_index.is_some() && (tag_name.eq("a") || tag_name.eq("A")) {
				no_nested_a_index = None;
			}
			let target = tag_name.to_lowercase();
			// try to find matching open
			let mut i = stack.len();
			while i > 1 {
				// skip root at 0
				i -= 1;
				if stack[i].elem.name().eq_ignore_ascii_case(&target) {
					while stack.len() > i + 1 {
						let closed = stack.pop().unwrap();
						let parent = stack.last_mut().unwrap();
						let mut e = closed.elem;
						let parent_ptr: *mut HTMLElement = &mut *parent.elem;
						e.parent = Some(parent_ptr);
						let close_end = full.end().saturating_sub(frame_offset);
						if e.range().is_some() {
							e.set_range_end(close_end);
						}
						parent.elem.children.push(Node::Element(e));
					}
					let closed = stack.pop().unwrap();
					let parent = stack.last_mut().unwrap();
					let mut e = closed.elem;
					let parent_ptr: *mut HTMLElement = &mut *parent.elem;
					e.parent = Some(parent_ptr);
					let close_end_main = full.end().saturating_sub(frame_offset);
					if e.range().is_some() {
						e.set_range_end(close_end_main);
					}
					parent.elem.children.push(Node::Element(e));
					break;
				} else {
					// aggressive strategy: if parent would be auto-closed by this closing tag
					let parent_name = stack[i].elem.name().to_lowercase();
					if let Some(list) = closed_by_close.get(parent_name.as_str()) {
						if list.iter().any(|x| x.eq_ignore_ascii_case(&tag_name)) {
							let closed = stack.pop().unwrap();
							let parent = stack.last_mut().unwrap();
							let mut e = closed.elem;
							let parent_ptr: *mut HTMLElement = &mut *parent.elem;
							e.parent = Some(parent_ptr);
							parent.elem.children.push(Node::Element(e));
							continue;
						}
					}
				}
			}
		}
	}
	// trailing text if any
	if last_text_pos < data.len() {
		let text = &data[last_text_pos..];
		if !text.is_empty() {
			let top = stack.last_mut().unwrap();
			let start_src = last_text_pos.saturating_sub(frame_offset);
			let end_src = data.len().saturating_sub(frame_offset);
			top.elem.children.push(Node::Text(TextNode::with_range(
				text.to_string(),
				start_src,
				end_src,
			)));
		}
	}
	// unwind unless parse_none_closed_tags keeps them as-is
	if opts.parse_none_closed_tags {
		// 保持原样直接线性挂接
		while stack.len() > 1 {
			let closed = stack.pop().unwrap();
			let parent = stack.last_mut().unwrap();
			let mut e = closed.elem;
			let parent_ptr: *mut HTMLElement = &mut *parent.elem;
			e.parent = Some(parent_ptr);
			parent.elem.children.push(Node::Element(e));
		}
		let root = stack.pop().unwrap().elem;
		return root;
	}
	// JS parse() 错误修复阶段：处理 pair error 与 single error
	// stack[0] 为 root, 其余为未闭合链。模拟 JS: while stack.length > 1 { let last = pop(); let oneBefore = back(); ... }
	while stack.len() > 1 {
		let last = stack.pop().unwrap();
		let one_before = stack.last_mut().unwrap();
		let mut last_elem = last.elem; // detached error element
								 // 简化 pair error 逻辑：若相邻未闭合标签标签名相同，视为 pair（与 JS 修复阶段行为保持）
		let is_pair = last_elem.name() == one_before.elem.name();
		if is_pair {
			// pair error 修复：移除 last，将其子节点上提到 oneBefore 的父级
			// 这里简化：直接附加到 one_before.elem 的父（若存在）
			// pair 修复：移除 one_before 的重复子元素，将 last 的子节点上移到 one_before 的父级
			if let Some(one_parent_ptr) = one_before.elem.parent {
				unsafe {
					let one_parent = &mut *one_parent_ptr;
					one_parent.remove_children_where(
						|n| matches!(n, Node::Element(e) if e.name()==last_elem.name()),
					);
					for mut child in last_elem.children.drain(..) {
						if let Node::Element(e) = &mut child {
							let parent_ptr: *mut HTMLElement = one_parent_ptr;
							e.parent = Some(parent_ptr);
						}
						one_parent.children.push(child);
					}
				}
			}
			continue;
		}
		// single error: remove last but keep its children inside one_before
		// 但若这是文件尾部未闭合的正常标签（比如 <div><p>abc），应保留 last_elem 并设置其 range 结束到末尾
		let end_fix_needed =
			last_elem.range().is_some() && last_elem.range().unwrap().1 < input.len();
		if end_fix_needed {
			// finalize range end to input end
			last_elem.set_range_end(input.len());
			// 挂回 one_before
			let parent_ptr: *mut HTMLElement = &mut *one_before.elem;
			last_elem.parent = Some(parent_ptr);
			one_before.elem.children.push(Node::Element(last_elem));
			continue;
		} else {
			let target_name = last_elem.name().to_string();
			one_before
				.elem
				.remove_children_where(|n| matches!(n, Node::Element(e) if e.name()==target_name));
			for mut child in last_elem.children.drain(..) {
				match &mut child {
					Node::Element(e) => {
						let parent_ptr: *mut HTMLElement = &mut *one_before.elem;
						e.parent = Some(parent_ptr);
					}
					_ => {}
				}
				one_before.elem.children.push(child);
			}
		}
	}
	// 最终收束：stack 剩 root
	let root = stack.pop().unwrap().elem;
	// 后处理：相邻重复 heading（h1-h6）时，将后者子节点（非空内容）提升为前者之后的兄弟，并移除后者。
	// 目的：复刻 JS parse() 在修复相邻或嵌套 heading 过程中产生的展平效果（见 tests 中 h3 链相关用例）。
	fn promote_heading_duplicates(node: &mut HTMLElement) {
		use crate::dom::node::Node;
		let mut i = 0;
		while i + 1 < node.children.len() {
			let promote = match (&node.children[i], &node.children[i + 1]) {
				(Node::Element(a), Node::Element(b)) => {
					let n1 = a.name();
					let n2 = b.name();
					if n1 == n2 && matches!(n1, "h1" | "h2" | "h3" | "h4" | "h5" | "h6") {
						Some((n1.to_string(), i + 1))
					} else {
						None
					}
				}
				_ => None,
			};
			// 如果当前是 heading，后面紧跟不同标签且当前 heading 没有结束 range，补一个空结束（使序列化产生 </h3>)
			if promote.is_none() && i + 1 < node.children.len() {
				// safe split to avoid aliasing mutable/immutable borrows
				let (left, right) = node.children.split_at_mut(i + 1); // left has up to i
				if let Some(Node::Element(h)) = left.last_mut() {
					if let Some(Node::Element(next_el)) = right.first() {
						let hn = h.name();
						if matches!(hn, "h1" | "h2" | "h3" | "h4" | "h5" | "h6")
							&& hn != next_el.name()
						{
							if let Some(r) = h.range() {
								if r.0 == r.1 {
									h.set_range_end(r.0);
								}
							}
						}
					}
				}
			}
			if let Some((_name, dup_idx)) = promote {
				// 取出重复 heading
				let mut dup = match node.children.remove(dup_idx) {
					Node::Element(e) => e,
					_ => unreachable!(),
				};
				// 过滤并提升其子节点：丢弃空元素（无属性 & 无子节点）
				let insertion_pos = i + 1; // 在原第一 heading 之后依次插入
				let mut promoted: Vec<Node> = Vec::new();
				for child in dup.children.drain(..) {
					let keep = match &child {
						Node::Element(e) => {
							let name = e.name();
							// 仅过滤“空 div” (无属性且无子节点)；其它标签即使空也保留（如 span）以匹配 js 行为
							if name == "div" && e.raw_attrs.is_empty() && e.children.is_empty() {
								false
							} else {
								true
							}
						}
						_ => true,
					};
					if keep {
						promoted.push(child);
					}
				}
				for (offset, mut ch) in promoted.into_iter().enumerate() {
					if let Node::Element(ref mut e) = ch {
						e.parent = Some(node as *mut HTMLElement);
					}
					node.children.insert(insertion_pos + offset, ch);
				}
				// 若第一个 heading 仍为空且 range 未闭合 (start==end)，为使序列化输出闭合标签，模拟设置结束位置
				if let Node::Element(first_h) = &mut node.children[i] {
					if matches!(first_h.name(), "h1" | "h2" | "h3" | "h4" | "h5" | "h6") {
						if let Some((s, e)) = first_h.range() {
							if s == e {
								first_h.set_range_end(s + 1);
							}
						}
					}
				}
				// 不递增 i，重新检查当前位置后续可能还有重复
				continue;
			}
			i += 1;
		}
		// 递归
		for child in node.children.iter_mut() {
			if let Node::Element(e) = child {
				promote_heading_duplicates(e);
			}
		}
	}
	let mut root_box = root;
	promote_heading_duplicates(&mut root_box);
	root_box
}

/// 验证HTML是否有效
///
/// 解析HTML并检查栈长度是否为1，类似于js/node-html-parser中的valid函数
///
/// # 参数
///
/// * `input` - 要解析的HTML字符串
/// * `opts` - 解析选项
///
/// # 返回值
///
/// 如果HTML有效（即栈长度为1）返回true，否则返回false
pub fn valid(input: &str, opts: &Options) -> bool {
	// 依据 JS 版本：仅看解析后栈是否仅包含 root
	// 这里实现一个轻量 base_parse，与上方 parse 主逻辑分离，尽量复刻 js/node-html-parser/nodes/html.ts 中 base_parse 的栈规则

	// void & frameflag 包装
	const FRAMEFLAG: &str = "documentfragmentcontainer";
	let data = format!("<{}>{}</{}>", FRAMEFLAG, input, FRAMEFLAG);
	// data_end_virtual 未使用，移除以消除编译警告

	let void_tag = VoidTag::new(&opts.void_tag);
	// 与 parse_with_options 同步的（精简）Unicode tag name 支持：
	let tag_re =
		Regex::new(r"<!--[\s\S]*?-->|<(\/)?([A-Za-z][-.:0-9_A-Za-z@\p{L}\p{M}]*)([^>]*)>").unwrap();

	// 对齐 JS 关闭规则映射
	use std::collections::HashMap; // 局部使用，避免与上面 parse 冲突
	let mut closed_by_open: HashMap<&'static str, HashMap<&'static str, bool>> = HashMap::new();
	macro_rules! c_open {($p:literal, [$($c:literal),*]) => {{ let mut m=HashMap::new(); $(m.insert($c,true);)* closed_by_open.insert($p,m.clone()); closed_by_open.insert($p.to_uppercase().leak(),m);}}}
	c_open!("li", ["li", "LI"]);
	c_open!("p", ["p", "P", "div", "DIV"]);
	c_open!("b", ["div", "DIV"]);
	c_open!("td", ["td", "th", "TD", "TH"]);
	c_open!("th", ["td", "th", "TD", "TH"]);
	c_open!("h1", ["h1", "H1"]);
	c_open!("h2", ["h2", "H2"]);
	c_open!("h3", ["h3", "H3"]);
	c_open!("h4", ["h4", "H4"]);
	c_open!("h5", ["h5", "H5"]);
	c_open!("h6", ["h6", "H6"]);

	let mut closed_by_close: HashMap<&'static str, HashMap<&'static str, bool>> = HashMap::new();
	macro_rules! c_close {($p:literal, [$($c:literal),*]) => {{ let mut m=HashMap::new(); $(m.insert($c,true);)* closed_by_close.insert($p,m.clone()); closed_by_close.insert($p.to_uppercase().leak(),m);}}}
	c_close!("li", ["ul", "ol", "UL", "OL"]);
	c_close!("a", ["div", "DIV"]);
	c_close!("b", ["div", "DIV"]);
	c_close!("i", ["div", "DIV"]);
	c_close!("p", ["div", "DIV"]);
	c_close!("td", ["tr", "table", "TR", "TABLE"]);
	c_close!("th", ["tr", "table", "TR", "TABLE"]);

	#[derive(Clone)]
	struct SimpleEl {
		raw: String,
	}
	let mut stack: Vec<SimpleEl> = vec![SimpleEl {
		raw: "#root".into(),
	}];
	// block text elements: 其内部视为原始文本，不再解析标签（与 JS 一致）
	let mut pos = 0usize;
	let block_text: std::collections::HashSet<&'static str> =
		["script", "style", "pre", "noscript"].into_iter().collect();
	while let Some(m) = tag_re.find_at(&data, pos) {
		let full = m.as_str();
		pos = m.end();
		if full.starts_with("<!--") {
			continue;
		}
		let caps = tag_re.captures(full).unwrap();
		let leading_slash = caps.get(1).map(|c| c.as_str()).unwrap_or("");
		let tag_name_raw = caps.get(2).map(|c| c.as_str()).unwrap_or("");
		let mut tag_name = tag_name_raw.to_string();
		let tag_name_lc = tag_name_raw.to_ascii_lowercase();
		if opts.lower_case_tag_name {
			tag_name = tag_name_lc.clone();
		}
		if tag_name_lc == FRAMEFLAG {
			continue;
		}
		let attr_part = caps.get(3).map(|c| c.as_str()).unwrap_or("");
		// 兼容 issue_227 与 frameflag 包装：若错误地把 frameflag 的关闭标签吞进属性（出现 '</documentfragmentcontainer'），说明这是跨越边界的伪匹配，跳过
		if leading_slash.is_empty() && attr_part.contains("</documentfragmentcontainer") {
			continue;
		}
		let self_close = attr_part.trim_end().ends_with('/') || void_tag.is_void(&tag_name_lc);
		if leading_slash.is_empty() {
			// opening tag
			if let Some(parent) = stack.last() {
				if let Some(map) = closed_by_open.get(parent.raw.as_str()) {
					if map.contains_key(tag_name.as_str()) {
						stack.pop();
					}
				}
			}
			if !self_close && !void_tag.is_void(&tag_name_lc) {
				let is_block = block_text.contains(tag_name_lc.as_str());
				stack.push(SimpleEl {
					raw: if opts.lower_case_tag_name {
						tag_name_lc.clone()
					} else {
						tag_name.clone()
					},
				});
				if is_block {
					// 查找对应关闭标签（大小写不敏感）
					let close_pat = format!("</{}>", tag_name_lc);
					// 为避免多次分配，做一次 to_lowercase() 子串搜索
					if let Some(rel_idx) = data[pos..].to_ascii_lowercase().find(&close_pat) {
						let close_start = pos + rel_idx; // '<' of closing
									   // 找到 '>'
						if let Some(gt_rel) = data[close_start..].find('>') {
							// pop block 元素（模拟遇到关闭标签）
							stack.pop();
							pos = close_start + gt_rel + 1; // 跳过整个关闭标签
							continue; // 继续下一轮 find_at
						} else {
							// 没有 '>'，视为未闭合，结束
							break;
						}
					} else {
						// 未找到关闭，视为未闭合 => 结束
						break;
					}
				}
			}
		} else {
			let target_lc = tag_name_lc.as_str();
			// closing tag
			loop {
				if let Some(top) = stack.last() {
					if top.raw.eq(target_lc) || top.raw.eq(tag_name.as_str()) {
						stack.pop();
						break;
					}
					if let Some(map) = closed_by_close.get(top.raw.as_str()) {
						if map.contains_key(tag_name.as_str()) || map.contains_key(target_lc) {
							stack.pop();
							continue;
						}
					}
				}
				break;
			}
		}
	}
	// JS: stack length==1 表明完整闭合
	let ok = stack.len() == 1;
	ok
}
