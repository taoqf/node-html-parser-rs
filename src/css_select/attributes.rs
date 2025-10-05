use crate::css_select::types::{Adapter, AttributeAction, AttributeSelector};

pub fn compose_attribute<'a, A: Adapter + 'a>(
	next: Box<dyn Fn(&A::HTMLElement) -> bool + 'a>,
	sel: &AttributeSelector,
	adapter: &'a A,
) -> Box<dyn Fn(&A::HTMLElement) -> bool + 'a> {
	let name = sel.name.clone();
	let action = sel.action.clone();
	let value = sel.value.clone();
	let ignore_case = sel.ignore_case;
	Box::new(move |el| {
		let attr = adapter.get_attribute(el, &name);
		match action {
			AttributeAction::Exists => {
				if attr.is_some() {
					return next(el);
				} else {
					return false;
				}
			}
			AttributeAction::Equals => {
				if let (Some(v), Some(val)) = (attr, value.as_ref()) {
					let (a, b) = if ignore_case {
						(v.to_ascii_lowercase(), val.to_ascii_lowercase())
					} else {
						(v.to_string(), val.to_string())
					};
					if a == b {
						return next(el);
					}
				}
				false
			}
			AttributeAction::Not => {
				if let (Some(v), Some(val)) = (attr, value.as_ref()) {
					let (a, b) = if ignore_case {
						(v.to_ascii_lowercase(), val.to_ascii_lowercase())
					} else {
						(v.to_string(), val.to_string())
					};
					if a == b {
						return false;
					}
				}
				next(el)
			}
			AttributeAction::Start => {
				if let (Some(v), Some(val)) = (attr, value.as_ref()) {
					let (a, b) = if ignore_case {
						(v.to_ascii_lowercase(), val.to_ascii_lowercase())
					} else {
						(v.to_string(), val.to_string())
					};
					if a.starts_with(&b) {
						return next(el);
					}
				}
				false
			}
			AttributeAction::End => {
				if let (Some(v), Some(val)) = (attr, value.as_ref()) {
					let (a, b) = if ignore_case {
						(v.to_ascii_lowercase(), val.to_ascii_lowercase())
					} else {
						(v.to_string(), val.to_string())
					};
					if a.ends_with(&b) {
						return next(el);
					}
				}
				false
			}
			AttributeAction::Any => {
				if let (Some(v), Some(val)) = (attr, value.as_ref()) {
					let (a, b) = if ignore_case {
						(v.to_ascii_lowercase(), val.to_ascii_lowercase())
					} else {
						(v.to_string(), val.to_string())
					};
					if a.contains(&b) {
						return next(el);
					}
				}
				false
			}
			AttributeAction::Hyphen => {
				if let (Some(v), Some(val)) = (attr, value.as_ref()) {
					let (a, b) = if ignore_case {
						(v.to_ascii_lowercase(), val.to_ascii_lowercase())
					} else {
						(v.to_string(), val.to_string())
					};
					if a == b || a.starts_with(&format!("{}-", b)) {
						return next(el);
					}
				}
				false
			}
			AttributeAction::Element => {
				if let (Some(v), Some(val)) = (attr, value.as_ref()) {
					let (a, b) = if ignore_case {
						(v.to_ascii_lowercase(), val.to_ascii_lowercase())
					} else {
						(v.to_string(), val.to_string())
					};
					if a.split_whitespace().any(|t| t == b) {
						return next(el);
					}
				}
				false
			}
		}
	})
}
