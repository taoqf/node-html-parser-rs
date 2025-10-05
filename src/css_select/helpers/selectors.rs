use crate::css_select::types::{AttributeAction, AttributeSelector, InternalSelector, PseudoData};

// Determine relative execution cost (lower = earlier) similar to js/css-select helpers/selectors.ts
pub fn get_quality(token: &InternalSelector) -> i32 {
	match token {
		InternalSelector::Universal => 50,
		InternalSelector::Tag { .. } => 30,
		InternalSelector::Attribute(attr) => get_attribute_quality(attr),
		InternalSelector::Pseudo { data, name } => match data {
			PseudoData::None => 3,
			PseudoData::SubSelectors(_) => {
				if matches!(name.as_str(), "has" | "contains" | "icontains") { 0 } else { 2 }
			}
			PseudoData::Nth(_) => 3, // nth 自身判定成本中等，先给 3（可再调优）
		}
		_ => -1,
	}
}

fn get_attribute_quality(attr: &AttributeSelector) -> i32 {
	let base = match attr.action {
		AttributeAction::Exists => 10,
		AttributeAction::Equals => {
			if attr.name == "id" {
				9
			} else {
				8
			}
		}
		AttributeAction::Not => 7,
		AttributeAction::Start | AttributeAction::End => 6,
		AttributeAction::Any => 5,
		AttributeAction::Hyphen => 4,
		AttributeAction::Element => 3,
	};
	if attr.ignore_case { base / 2 } else { base }
}

pub fn sort_rules(arr: &mut Vec<InternalSelector>) {
	// insertion sort by quality ascending (negative qualities stay in place)
	for i in 1..arr.len() {
		let q_new = get_quality(&arr[i]);
		if q_new < 0 {
			continue;
		}
		let mut j = i;
		while j > 0 {
			let q_prev = get_quality(&arr[j - 1]);
			if q_prev <= q_new {
				break;
			}
			arr.swap(j, j - 1);
			j -= 1;
		}
	}
}

pub fn is_traversal(token: &InternalSelector) -> bool {
	matches!(
		token,
		InternalSelector::Descendant
			| InternalSelector::Child
			| InternalSelector::Sibling
			| InternalSelector::Adjacent
			| InternalSelector::FlexibleDescendant
	)
}

pub fn includes_scope_pseudo(token: &InternalSelector) -> bool {
	match token {
		InternalSelector::Pseudo { name, data } => {
			if name == "scope" { return true; }
			match data {
				PseudoData::SubSelectors(groups) => groups.iter().any(|g| g.iter().any(includes_scope_pseudo)),
				_ => false,
			}
		}
		_ => false,
	}
}
