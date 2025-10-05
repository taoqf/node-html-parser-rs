use crate::css_select::legacy::{AttrOp, Combinator, CompoundSelector, Pseudo, Selector};
use crate::css_select::types::{AttributeAction, AttributeSelector, InternalSelector, PseudoData};

// 修复点：
// 1. 生成顺序与 JS 版本保持一致 (左->右：左侧 simple tokens, combinator, 右侧 simple tokens)。
// 2. 将 :not(...) 的内部 selector 转成 data，避免信息丢失；其它函数伪类暂保持 data=None。

fn simple_pseudo(name: &str) -> InternalSelector {
	InternalSelector::Pseudo {
		name: name.into(),
		data: PseudoData::None,
	}
}

// 转换单个复合选择器 (不含连接符)
fn compound_to_internal(comp: &CompoundSelector) -> Vec<InternalSelector> {
	let mut out = Vec::new();
	if let Some(tag) = &comp.tag {
		out.push(InternalSelector::Tag { name: tag.clone() });
	}
	if let Some(id) = &comp.id {
		out.push(InternalSelector::Attribute(AttributeSelector {
			name: "id".into(),
			action: AttributeAction::Equals,
			value: Some(id.clone()),
			ignore_case: false,
		}));
	}
	for cls in &comp.classes {
		out.push(InternalSelector::Attribute(AttributeSelector {
			name: "class".into(),
			action: AttributeAction::Element,
			value: Some(cls.clone()),
			ignore_case: false,
		}));
	}
	for a in &comp.attrs {
		let action = match a.op {
			AttrOp::Exists => AttributeAction::Exists,
			AttrOp::Eq => AttributeAction::Equals,
			AttrOp::Prefix => AttributeAction::Start,
			AttrOp::Suffix => AttributeAction::End,
			AttrOp::Substr => AttributeAction::Any,
			AttrOp::Includes => AttributeAction::Element,
			AttrOp::Dash => AttributeAction::Hyphen,
		};
		out.push(InternalSelector::Attribute(AttributeSelector {
			name: a.name.clone(),
			action,
			value: if matches!(a.op, AttrOp::Exists) {
				None
			} else {
				Some(a.value.clone())
			},
			ignore_case: matches!(a.case, crate::css_select::legacy::CaseMode::Insensitive),
		}));
	}
	for p in &comp.pseudos {
		match p {
			Pseudo::Not(list) => {
				let mut groups: Vec<Vec<InternalSelector>> = Vec::new();
				for sel in list {
					for seq in selector_to_internal(sel) {
						groups.push(seq);
					}
				}
				out.push(InternalSelector::Pseudo {
					name: "not".into(),
					data: PseudoData::SubSelectors(groups),
				});
			}
			Pseudo::FirstChild => out.push(simple_pseudo("first-child")),
			Pseudo::LastChild => out.push(simple_pseudo("last-child")),
			Pseudo::OnlyChild => out.push(simple_pseudo("only-child")),
			Pseudo::FirstOfType => out.push(simple_pseudo("first-of-type")),
			Pseudo::LastOfType => out.push(simple_pseudo("last-of-type")),
			Pseudo::OnlyOfType => out.push(simple_pseudo("only-of-type")),
			Pseudo::NthChild(expr) => out.push(InternalSelector::Pseudo {
				name: "nth-child".into(),
				data: PseudoData::Nth(expr.clone()),
			}),
			Pseudo::NthLastChild(expr) => out.push(InternalSelector::Pseudo {
				name: "nth-last-child".into(),
				data: PseudoData::Nth(expr.clone()),
			}),
			Pseudo::NthOfType(expr) => out.push(InternalSelector::Pseudo {
				name: "nth-of-type".into(),
				data: PseudoData::Nth(expr.clone()),
			}),
			Pseudo::NthLastOfType(expr) => out.push(InternalSelector::Pseudo {
				name: "nth-last-of-type".into(),
				data: PseudoData::Nth(expr.clone()),
			}),
			Pseudo::Is(list) => {
				let mut groups = Vec::new();
				for sel in list {
					for seq in selector_to_internal(sel) {
						groups.push(seq);
					}
				}
				out.push(InternalSelector::Pseudo {
					name: "is".into(),
					data: PseudoData::SubSelectors(groups),
				});
			}
			Pseudo::Where(list) => {
				let mut groups = Vec::new();
				for sel in list {
					for seq in selector_to_internal(sel) {
						groups.push(seq);
					}
				}
				out.push(InternalSelector::Pseudo {
					name: "where".into(),
					data: PseudoData::SubSelectors(groups),
				});
			}
			Pseudo::Has(list) => {
				let mut groups = Vec::new();
				for sel in list {
					for seq in selector_to_internal(sel) {
						groups.push(seq);
					}
				}
				out.push(InternalSelector::Pseudo {
					name: "has".into(),
					data: PseudoData::SubSelectors(groups),
				});
			}
			Pseudo::Empty => out.push(simple_pseudo("empty")),
			Pseudo::Root => out.push(simple_pseudo("root")),
			Pseudo::Scope => out.push(simple_pseudo("scope")),
		}
	}
	out
}

pub fn selector_to_internal(sel: &Selector) -> Vec<Vec<InternalSelector>> {
	let mut seq: Vec<InternalSelector> = Vec::new();
	let parts = &sel.0;
	if parts.is_empty() {
		return vec![seq];
	}
	for (idx, (_comb, comp)) in parts.iter().enumerate() {
		let mut simple_tokens = compound_to_internal(comp);
		seq.append(&mut simple_tokens);
		if let Some((Some(next_comb), _)) = parts.get(idx + 1) {
			let comb_token = match next_comb {
				Combinator::Descendant => InternalSelector::Descendant,
				Combinator::Child => InternalSelector::Child,
				Combinator::Adjacent => InternalSelector::Adjacent,
				Combinator::Sibling => InternalSelector::Sibling,
			};
			seq.push(comb_token);
		}
	}
	vec![seq]
}
