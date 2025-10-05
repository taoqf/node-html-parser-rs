use super::main::HTMLElement;
use crate::dom::node::Node;

// （占位）未来若需要对子节点 map/reduce 并行，可在启用 parallel 特性时引入 rayon::ParallelIterator。
#[cfg(feature = "parallel")]
use rayon::prelude::*; // 预留：未来可将同级 Element 子树收集并行化

impl HTMLElement {
	pub fn structured_text(&self) -> String {
		use std::collections::HashSet;
		use std::sync::OnceLock;
		static BLOCK: OnceLock<HashSet<&'static str>> = OnceLock::new();
		let block = BLOCK.get_or_init(|| {
			[
				"h1",
				"h2",
				"h3",
				"h4",
				"h5",
				"h6",
				"header",
				"hgroup",
				"details",
				"dialog",
				"dd",
				"div",
				"dt",
				"fieldset",
				"figcaption",
				"figure",
				"footer",
				"form",
				"table",
				"td",
				"tr",
				"address",
				"article",
				"aside",
				"blockquote",
				"br",
				"hr",
				"li",
				"main",
				"nav",
				"ol",
				"p",
				"pre",
				"section",
				"ul",
			]
			.into_iter()
			.collect()
		});
		// Each block: collected text fragments plus optional postponed whitespace flag
		#[derive(Default)]
		struct LineBlock {
			parts: Vec<String>,
			prepend_ws: bool,
		}
		let mut blocks: Vec<LineBlock> = vec![LineBlock::default()];
		// 并行化策略：对同级子节点（Element/Text）收集片段时并行映射，再按顺序合并。
		fn dfs(
			cur: &HTMLElement,
			block: &std::collections::HashSet<&'static str>,
		) -> Vec<LineBlock> {
			let tag = cur.name();
			let is_block =
				!cur.is_root() && (block.contains(tag) || block.contains(&tag.to_lowercase()[..]));
			let children = &cur.children;
			// 收集子节点处理结果
			let mut acc: Vec<LineBlock> = Vec::new();
			let mut current = LineBlock::default();
			// 为保持顺序：仍按序遍历，但对 Element 内部递归结果使用 maybe_par_iter 预先收集（子树内部可再并行）。
			for child in children {
				match child {
					Node::Element(e) => {
						let cname = e.name();
						let child_block =
							block.contains(cname) || block.contains(&cname.to_lowercase()[..]);
						if child_block && !current.parts.is_empty() {
							acc.push(current);
							current = LineBlock::default();
						}
						let sub_blocks = dfs(e, block); // 递归（内部自会并行展开）
						for (i, sb) in sub_blocks.into_iter().enumerate() {
							if i == 0 {
								// 第一块并入 current
								if current.prepend_ws && !sb.parts.is_empty() {
									current.parts.push(format!(" {}", sb.parts.join("")));
									current.prepend_ws = false;
								} else {
									current.parts.extend(sb.parts);
								}
								if sb.prepend_ws {
									current.prepend_ws = true;
								}
							} else {
								acc.push(current);
								current = sb; // 将之前 current 推入，接手子块
							}
						}
						if child_block && !current.parts.is_empty() {
							acc.push(current);
							current = LineBlock::default();
						}
					}
					Node::Text(t0) => {
						if t0.is_whitespace() {
							current.prepend_ws = true;
							continue;
						}
						let mut tc = t0.clone();
						let txt = tc.trimmed_text().to_string();
						if current.prepend_ws {
							current.parts.push(format!(" {}", txt));
							current.prepend_ws = false;
						} else {
							current.parts.push(txt);
						}
					}
					Node::Comment(_) => {}
				}
			}
			if !current.parts.is_empty() {
				acc.push(current);
			}
			if is_block {
				acc.push(LineBlock::default());
			}
			acc
		}
		let mut collected = dfs(self, block);
		blocks.append(&mut collected);
		blocks
			.into_iter()
			.filter(|b| !b.parts.is_empty())
			.map(|b| {
				let joined = b.parts.join("");
				regex::Regex::new(r"\s{2,}")
					.unwrap()
					.replace_all(&joined, " ")
					.to_string()
			})
			.collect::<Vec<_>>()
			.join("\n")
			.trim_end()
			.to_string()
	}

	fn collect_structured_text(&self, buf: &mut String, is_root: bool) {
		for (i, child) in self.children.iter().enumerate() {
			match child {
				Node::Text(t) => {
					let txt = html_escape::decode_html_entities(&t.raw);
					if !txt.trim().is_empty() {
						buf.push_str(txt.trim());
						if i + 1 < self.children.len() {
							buf.push('\n');
						}
					}
				}
				Node::Element(e) => {
					e.collect_structured_text(buf, false);
				}
				Node::Comment(_) => {}
			}
		}
	}
}
