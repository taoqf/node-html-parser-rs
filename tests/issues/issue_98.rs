use node_html_parser::parse;

#[test]
fn issue_98_get_attribute_case_variants() {
	let root = parse("<a onclick=\"listener()\"></a>");
	let a = root.query_selector("a").unwrap();
	assert_eq!(a.get_attr("onclick"), Some("listener()"));
}

#[test]
fn issue_98_get_attribute_original_casing() {
	let root = parse("<a onClick=\"listener()\"></a>");
	let a = root.query_selector("a").unwrap();
	assert_eq!(a.get_attr("onClick"), Some("listener()"));
	assert_eq!(a.get_attr("onclick"), Some("listener()"));
}

#[test]
fn issue_98_set_attribute_lowercase_and_additional() {
	let root = parse("<a onClick=\"listener()\"></a>");
	let a = root.query_selector("a").unwrap();
	let mut clone = a.clone_node();
	clone.set_attribute("onclick", "listener2");
	assert_eq!(clone.get_attr("onclick"), Some("listener2"));
	clone.set_attribute("onDoubleClick", "listener3");
	assert_eq!(clone.get_attr("onDoubleClick"), Some("listener3"));
	// outer_html of clone (case may differ based on canonicalization logic)
	assert!(clone.outer_html().contains("onClick=\"listener2\""));
	assert!(clone.outer_html().contains("onDoubleClick=\"listener3\""));
}
