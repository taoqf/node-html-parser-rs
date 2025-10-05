use node_html_parser::{parse_with_options, Options};

// issue 165: getElementById 对包含 '.' 与 空格的 id
#[test]
fn issue_165_get_element_by_id_dot_and_space() {
	let root = parse_with_options(
		"<div><foo id=\"foo.bar\">bar</foo></div>",
		&Options::default(),
	);
	let div = root.first_element_child().unwrap();
	// query_selector('#foo.bar') 应该解释为 id=foo 且 class 包含 bar，不匹配本 element
	assert!(div.query_selector("#foo.bar").is_none());
	let foo = div.query_selector("foo").unwrap();
	let by_id = div.get_element_by_id("foo.bar").unwrap();
	assert!(std::ptr::eq(foo as *const _, by_id as *const _));

	let root2 = parse_with_options(
		"<div><foo id=\"foo bar\">bar</foo></div>",
		&Options::default(),
	);
	let div2 = root2.first_element_child().unwrap();
	assert!(div2.query_selector("#foo bar").is_none());
	let foo2 = div2.query_selector("foo").unwrap();
	let by_id2 = div2.get_element_by_id("foo bar").unwrap();
	assert!(std::ptr::eq(foo2 as *const _, by_id2 as *const _));
}
