use super::convert::selector_to_internal;
use super::helpers::selectors::{is_traversal, sort_rules};
use super::legacy::{apply_selector_from_el, parse_selector_list_public};
use super::types::{Adapter, CompiledQuery, Options};
use crate::css_select::general::compile_general_selector;
use crate::css_select::types::InternalSelector;
use crate::dom::element::HTMLElement;

// 兼容旧实现：仍保留 legacy 后端 (稳定可靠)，新管线逐步接入。
// compile_token 继续走 legacy，防止现有对外 API 行为突变。
pub fn compile_token<'a, A: Adapter>(
	selector: &str,
	_options: &Options<A>,
	root: &'a HTMLElement,
) -> CompiledQuery<'a, A> {
	let legacy_list = parse_selector_list_public(selector);
	CompiledQuery::new(move |el: &A::HTMLElement| {
		let real = el as *const A::HTMLElement as *const HTMLElement;
		let real_ref = unsafe { &*real };
		for sel in &legacy_list {
			if apply_selector_from_el(root, real_ref, sel) {
				return true;
			}
		}
		false
	})
}

// ----------------------------- 新编译管线 (试验性) -----------------------------
// 说明：
// 1. 使用 legacy 解析 -> convert.rs 生成 InternalSelector 序列。
// 2. 对每条序列按 traversal 边界分组并对组内 simple selectors 调用 sort_rules。
// 3. 按顺序调用 compile_general_selector 构建谓词链。
// 4. 多条 selector 之间 OR 组合。
// 5. 暂未实现 :nth-* / 复杂伪类 runtime，相关伪类会在 general.rs 中返回 false（除 :empty）。

fn reorder_with_sort(seq: &[InternalSelector]) -> Vec<InternalSelector> {
	let mut out: Vec<InternalSelector> = Vec::with_capacity(seq.len());
	let mut buffer: Vec<InternalSelector> = Vec::new();
	for tok in seq {
		if is_traversal(tok) {
			if !buffer.is_empty() {
				sort_rules(&mut buffer);
				out.extend(buffer.drain(..));
			}
			out.push(tok.clone());
		} else {
			buffer.push(tok.clone());
		}
	}
	if !buffer.is_empty() {
		sort_rules(&mut buffer);
		out.extend(buffer.drain(..));
	}
	out
}

pub fn compile_internal_new<'a, A: Adapter + 'a>(
	selector: &str,
	adapter: &'a A,
) -> CompiledQuery<'a, A> {
	let parsed = parse_selector_list_public(selector);
	if parsed.is_empty() {
		return CompiledQuery::new(|_| false);
	}

	// 针对每个逗号分隔 selector 构建一条链，最后 OR。
	let mut chains: Vec<Box<dyn Fn(&A::HTMLElement) -> bool + 'a>> = Vec::new();
	for sel in parsed {
		let internal_groups = selector_to_internal(&sel); // Vec<Vec<InternalSelector>>; 当前每个 sel 只返回单一序列
		for seq in internal_groups {
			if seq.is_empty() {
				continue;
			}
			// 相对选择器绝对化（简化版）：若未出现 scope 且首 token 非 traversal，则在链前插入 :scope Descendant
			let mut seq_work = seq.clone();
			let has_scope = seq_work
				.iter()
				.any(|t| matches!(t, InternalSelector::Pseudo { name, .. } if name == "scope"));
			if !has_scope {
				if !seq_work.first().map(|t| is_traversal(t)).unwrap_or(false) {
					seq_work.insert(0, InternalSelector::Descendant); // descendant between scope & rest
					seq_work.insert(
						0,
						InternalSelector::Pseudo {
							name: "scope".into(),
							data: crate::css_select::types::PseudoData::None,
						},
					);
				}
			}
			let reordered = reorder_with_sort(&seq_work);
			let mut next: Box<dyn Fn(&A::HTMLElement) -> bool + 'a> = Box::new(|_| true);
			for token in reordered.iter() {
				next = compile_general_selector(next, token, adapter);
				// 若某环节已成为永真 (未变化) 或永假 (目前不会出现) 可做早期优化，暂略。
			}
			chains.push(next);
		}
	}
	if chains.is_empty() {
		return CompiledQuery::new(|_| false);
	}
	let combined = move |el: &A::HTMLElement| {
		for f in &chains {
			if f(el) {
				return true;
			}
		}
		false
	};
	CompiledQuery::new(combined)
}
