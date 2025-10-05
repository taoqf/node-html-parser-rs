use node_html_parser::parse;

#[test]
fn issue_224_complex_nth_of_type_selector() {
	let html = std::fs::read_to_string("tests/assets/html/melon.html").expect("melon.html");
	let root = parse(&html);
	let h2 = root
		.query_selector(".band:nth-of-type(3) .col-md-4:nth-of-type(2) h2")
		.expect("selector h2");
	assert_eq!(h2.name().to_ascii_uppercase(), "H2");
}
