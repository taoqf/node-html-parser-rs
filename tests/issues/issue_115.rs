use node_html_parser::parse;

// Port of js/tests/issues/115.js
#[test]
fn issue_115_parse_html_inner_text() {
	let html = "<p>Hello <strong>world</strong>!</p>";
	let root = parse(html);
	let p = root.first_element_child().unwrap();
	// JS: innerText should collapse tags and show textual content
	// We approximate using structured text: text_content / inner_html behavior
	assert_eq!(p.text_content(), "Hello world!");
}
