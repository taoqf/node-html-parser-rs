use node_html_parser::parse;

#[test]
fn issue_42_svg_attr_with_namespace_prefix() {
	let root = parse(
		r#"<p a=12 data-id="!$$&amp;" yAz='1' xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"></p>"#,
	);
	let p = root.query_selector("p").unwrap();
	// get_attr is case-insensitive and returns decoded value
	assert_eq!(
		p.get_attr("xmlns:xlink"),
		Some("http://www.w3.org/1999/xlink")
	);
}
