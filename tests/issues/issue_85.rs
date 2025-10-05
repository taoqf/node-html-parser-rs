use node_html_parser::{parse, Node};

// issue 85: element removal behaviors
#[test]
fn issue_85_remove_current_element() {
	let mut root = parse("<div><a id=el></a></div>");
	// Locate <a> mutably and call remove()
	if let Some(div) = root.first_element_child_mut() {
		if let Some(pos) = div
			.children
			.iter()
			.position(|n| matches!(n, Node::Element(e) if e.id=="el"))
		{
			if let Node::Element(ref mut a) = div.children[pos] {
				a.remove();
			}
		}
	}
	assert_eq!(root.to_string(), "<div></div>");
}

#[test]
fn issue_85_remove_element_not_in_html() {
	let mut root = parse("<div></div><a id=el></a>");
	// Root children are at top-level container; find second element <a>
	if let Some(pos) = root
		.children
		.iter()
		.position(|n| matches!(n, Node::Element(e) if e.id=="el"))
	{
		if let Node::Element(ref mut a) = root.children[pos] {
			a.remove();
		}
	}
	// Serialization should no longer include <a>
	assert_eq!(root.to_string(), "<div></div>");
}

#[test]
fn issue_85_manual_filter_removal() {
	let mut root = parse("<div><a id=el></a></div>");
	if let Some(div) = root.first_element_child_mut() {
		div.children
			.retain(|n| !matches!(n, Node::Element(e) if e.id=="el"));
		assert!(!div
			.children
			.iter()
			.any(|n| matches!(n, Node::Element(e) if e.id=="el")));
	}
}
