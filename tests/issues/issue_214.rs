use node_html_parser::parse;

#[test]
fn issue_214_table_tagname_uppercase() {
	let html = std::fs::read_to_string("tests/assets/html/codes.html").expect("codes.html");
	let root = parse(&html);
	let table = root
		.query_selector("table.restable")
		.expect("table.restable");
	assert_eq!(table.name().to_ascii_uppercase(), "TABLE");
}
