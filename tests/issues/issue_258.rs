use node_html_parser::parse;

#[test]
fn issue_258_remove_attribute_boolean_rendering() {
	let _root = parse("<input>");
	// 重新解析用于可变克隆
	let root2 = parse("<input>");
	let input2 = root2.query_selector("input").unwrap();
	// SAFETY: create a mutable clone to perform attribute operations
	let mut cloned = input2.clone_node();
	cloned.set_attribute("checked", "");
	cloned.set_attribute("a", "");
	assert_eq!(cloned.outer_html(), "<input checked a>");
	cloned.remove_attribute("a");
	assert_eq!(cloned.outer_html(), "<input checked>");
}
