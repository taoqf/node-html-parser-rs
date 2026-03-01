//! HTML validation engine for checking well-formed markup.

use crate::dom::void_tag::VoidTag;
use regex::Regex;
use std::sync::OnceLock;

use super::types::Options;

static VALID_TAG_REGEX: OnceLock<Regex> = OnceLock::new();

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
	let tag_re = VALID_TAG_REGEX.get_or_init(|| {
		Regex::new(r"<!--[\s\S]*?-->|<(\/)?([A-Za-z][-.:0-9_A-Za-z@\p{L}\p{M}]*)([^>]*)>").unwrap()
	});

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
