use node_html_parser::parse;

#[test]
fn issue_239_basic_query_serialization() {
	let root = parse("<ul id=\"list\"><li>Hello World</li></ul>");
	let ul = root.query_selector("#list").unwrap();
	assert_eq!(ul.to_string(), "<ul id=\"list\"><li>Hello World</li></ul>");
	let li = root.query_selector("li").unwrap();
	assert_eq!(li.to_string(), "<li>Hello World</li>");
}
