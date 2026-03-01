use node_html_parser::{parse_with_options, Options};

#[test]
fn remove_whitespace_basic() {
	let root = parse_with_options(
		"<p> \r \n  \t <h5>  123&nbsp;  </h5></p>",
		&Options::default(),
	);
	let mut cloned = root.clone();
	cloned.remove_whitespace();
	// 期望内层文本被 trim_right / trim_left 后等价于 JS 行为
	let ser = cloned.outer_html();
	assert!(ser.contains("123&nbsp;"));
}

#[test]
fn preserve_meaningful_whitespace() {
	let mut root = parse_with_options(
		"<p>\t\n  Hello \n\t<em>World</em>!</p>",
		&Options::default(),
	);
	root.remove_whitespace();
	let p = root.first_element_child().unwrap();
	assert!(p.text_content().contains(" Hello World!"));
}
