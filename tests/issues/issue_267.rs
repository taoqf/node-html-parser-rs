use node_html_parser::parse;

#[test]
fn issue_267_empty_class_attribute_preserved() {
	// 使用原始字符串包裹，末尾包含空格
	let html = r##"<polygon points="-235.18 1571.95 1014.73 1284.4 1083.46 1590.1 -166.45 1877.65 -235.18 1571.95" fill="#ff8200" class="" design-color="primary"></polygon> "##;
	let root = parse(html);
	assert_eq!(root.outer_html(), html);
}
