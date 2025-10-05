use crate::css_select::attributes::compose_attribute;
use crate::css_select::legacy::NthExpr;
use crate::css_select::types::{Adapter, InternalSelector, PseudoData};

pub fn compile_general_selector<'a, A: Adapter + 'a>(
	next: Box<dyn Fn(&A::HTMLElement) -> bool + 'a>,
	selector: &InternalSelector,
	adapter: &'a A,
) -> Box<dyn Fn(&A::HTMLElement) -> bool + 'a> {
	match selector {
		InternalSelector::Universal => next,
		InternalSelector::Tag { name } => {
			let name = name.clone();
			Box::new(move |el| adapter.get_name(el) == name && next(el))
		}
		InternalSelector::Attribute(sel) => compose_attribute(next, sel, adapter),
		InternalSelector::Descendant => Box::new(move |el| {
			let mut current = adapter.get_parent(el);
			while let Some(p) = current {
				if next(p) {
					return true;
				}
				current = adapter.get_parent(p);
			}
			false
		}),
		InternalSelector::Child => {
			Box::new(move |el| adapter.get_parent(el).map(|p| next(p)).unwrap_or(false))
		}
		InternalSelector::Sibling => Box::new(move |el| {
			let sibs = adapter.get_siblings(el);
			for s in sibs {
				if adapter.equals(s, el) {
					break;
				}
				if next(s) {
					return true;
				}
			}
			false
		}),
		InternalSelector::Adjacent => Box::new(move |el| {
			let sibs = adapter.get_siblings(el);
			let mut last: Option<&A::HTMLElement> = None;
			for s in sibs {
				if adapter.equals(s, el) {
					break;
				}
				last = Some(s);
			}
			last.map(|l| next(l)).unwrap_or(false)
		}),
		InternalSelector::FlexibleDescendant => Box::new(move |el| {
			let mut cur: Option<&A::HTMLElement> = Some(el);
			while let Some(c) = cur {
				if next(c) {
					return true;
				}
				cur = adapter.get_parent(c);
			}
			false
		}),
		InternalSelector::Pseudo { name, data } => {
			// 预编译子选择器（:is/:where/:not/:has）子序列为闭包列表，避免每次重新构建
			let n = name.clone();
			let d = data.clone();
			let compiled_sub: Option<Vec<Box<dyn Fn(&A::HTMLElement) -> bool + 'a>>> = match &d {
				PseudoData::SubSelectors(groups) => {
					let mut list = Vec::with_capacity(groups.len());
					for seq in groups {
						// （移除调试打印）
						let mut inner: Box<dyn Fn(&A::HTMLElement) -> bool + 'a> =
							Box::new(|_| true);
						// 对子序列简单 tokens 排序（忽略 traversal 拆分，因为内部多为复合）
						let mut seq_clone = seq.clone();
						crate::css_select::helpers::selectors::sort_rules(&mut seq_clone);
						for tok in &seq_clone {
							inner = compile_general_selector(inner, tok, adapter);
						}
						list.push(inner);
					}
					Some(list)
				}
				_ => None,
			};
			Box::new(move |el| match n.as_str() {
				"scope" => next(el),
				"empty" => {
					if adapter.is_tag(el) && adapter.is_empty(el) {
						return next(el);
					}
					false
				}
				"first-child" => is_first_child(el, adapter) && next(el),
				"last-child" => is_last_child(el, adapter) && next(el),
				"only-child" => is_only_child(el, adapter) && next(el),
				"first-of-type" => is_first_of_type(el, adapter) && next(el),
				"last-of-type" => is_last_of_type(el, adapter) && next(el),
				"only-of-type" => is_only_of_type(el, adapter) && next(el),
				"root" => adapter.get_parent(el).is_none() && next(el),
				"not" => {
					if let Some(subs) = &compiled_sub {
						for f in subs {
							if f(el) {
								return false;
							}
						}
						return next(el);
					}
					next(el)
				}
				"is" | "where" => {
					if let Some(subs) = &compiled_sub {
						for (_idx, f) in subs.iter().enumerate() {
							let matched = f(el);
							if matched {
								return next(el);
							}
						}
						return false;
					}
					false
				}
				"has" => {
					if let Some(subs) = &compiled_sub {
						if has_descendant_with_any(el, adapter, subs) {
							return next(el);
						}
						return false;
					}
					false
				}
				"nth-child" | "nth-last-child" | "nth-of-type" | "nth-last-of-type" => match &d {
					PseudoData::Nth(expr) => {
						if match_nth_pseudo(el, adapter, n.as_str(), expr) {
							return next(el);
						}
						false
					}
					_ => false,
				},
				_ => false,
			})
		}
	}
}

// -------- 辅助：结构伪类判定（贴近 JS pseudos.ts 实现） --------
fn is_first_child<A: Adapter>(el: &A::HTMLElement, adapter: &A) -> bool {
	let siblings = adapter.get_siblings(el);
	for s in siblings {
		if adapter.is_tag(s) {
			return adapter.equals(s, el);
		}
	}
	false
}
fn is_last_child<A: Adapter>(el: &A::HTMLElement, adapter: &A) -> bool {
	let siblings = adapter.get_siblings(el);
	for s in siblings.iter().rev() {
		if adapter.is_tag(s) {
			return adapter.equals(s, el);
		}
	}
	false
}
fn is_only_child<A: Adapter>(el: &A::HTMLElement, adapter: &A) -> bool {
	let siblings = adapter.get_siblings(el);
	let mut found_other = false;
	for s in siblings {
		if adapter.is_tag(s) && !adapter.equals(s, el) {
			found_other = true;
			break;
		}
	}
	!found_other
}
fn is_first_of_type<A: Adapter>(el: &A::HTMLElement, adapter: &A) -> bool {
	let siblings = adapter.get_siblings(el);
	let name = adapter.get_name(el);
	for s in siblings {
		if adapter.equals(s, el) {
			return true;
		}
		if adapter.is_tag(s) && adapter.get_name(s) == name {
			break;
		}
	}
	false
}
fn is_last_of_type<A: Adapter>(el: &A::HTMLElement, adapter: &A) -> bool {
	let siblings = adapter.get_siblings(el);
	let name = adapter.get_name(el);
	for s in siblings.iter().rev() {
		if adapter.equals(s, el) {
			return true;
		}
		if adapter.is_tag(s) && adapter.get_name(s) == name {
			break;
		}
	}
	false
}
fn is_only_of_type<A: Adapter>(el: &A::HTMLElement, adapter: &A) -> bool {
	let siblings = adapter.get_siblings(el);
	let name = adapter.get_name(el);
	let mut others = false;
	for s in siblings {
		if adapter.is_tag(s) && !adapter.equals(s, el) && adapter.get_name(s) == name {
			others = true;
			break;
		}
	}
	!others
}

// 评估 :not 内部序列（简易：顺序执行 simple tokens；若含 traversal 当前实现不足但已有基础）
// 已弃用的 eval_simple_chain 被预编译子选择器取代

fn match_nth(expr: &NthExpr, index_one: i32) -> bool {
	match expr {
		NthExpr::Number(n) => index_one == *n,
		NthExpr::Odd => index_one % 2 == 1,
		NthExpr::Even => index_one % 2 == 0,
		NthExpr::Pattern { a, b } => {
			let a = *a;
			let b = *b;
			if a == 0 {
				return index_one == b;
			}
			if a > 0 {
				if index_one < b {
					return false;
				}
				(index_one - b) % a == 0
			} else {
				let mut k = 0;
				loop {
					let val = a * k + b;
					if val == index_one {
						return true;
					}
					if val < 1 || val > index_one {
						return false;
					}
					k += 1;
				}
			}
		}
	}
}

fn match_nth_pseudo<A: Adapter>(
	el: &A::HTMLElement,
	adapter: &A,
	name: &str,
	expr: &NthExpr,
) -> bool {
	let siblings = adapter.get_siblings(el);
	let mut positions: Vec<&A::HTMLElement> = Vec::new();
	match name {
		"nth-child" | "nth-last-child" => {
			for s in &siblings {
				if adapter.is_tag(s) {
					positions.push(*s);
				}
			}
			if name == "nth-child" {
				if let Some(pos) = positions.iter().position(|p| adapter.equals(p, el)) {
					return match_nth(expr, (pos as i32) + 1);
				}
			} else {
				if let Some(pos) = positions.iter().rposition(|p| adapter.equals(p, el)) {
					let rev = positions.len() - pos;
					return match_nth(expr, rev as i32);
				}
			}
		}
		"nth-of-type" | "nth-last-of-type" => {
			let tag = adapter.get_name(el);
			for s in &siblings {
				if adapter.is_tag(s) && adapter.get_name(s) == tag {
					positions.push(*s);
				}
			}
			if name == "nth-of-type" {
				if let Some(pos) = positions.iter().position(|p| adapter.equals(p, el)) {
					return match_nth(expr, (pos as i32) + 1);
				}
			} else {
				if let Some(pos) = positions.iter().rposition(|p| adapter.equals(p, el)) {
					let rev = positions.len() - pos;
					return match_nth(expr, rev as i32);
				}
			}
		}
		_ => {}
	}
	false
}

fn has_descendant_with_any<'a, A: Adapter>(
	el: &A::HTMLElement,
	adapter: &A,
	funcs: &Vec<Box<dyn Fn(&A::HTMLElement) -> bool + 'a>>,
) -> bool {
	for c in adapter.get_children(el) {
		for f in funcs {
			if f(&c) {
				return true;
			}
		}
		if has_descendant_with_any(c, adapter, funcs) {
			return true;
		}
	}
	false
}
