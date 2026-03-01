use node_html_parser::parse;

#[test]
fn issue_274_remove_whitespace_should_preserve_inside_tags() {
	let html = "<!DOCTYPE html\n><html                     lang=\"en\"\n><meta charset=\"UTF-8\"\n><title>test</title\n\n><p>test</p\n\n></html>";
	let root = parse(html);
	let html_el = root.query_selector("html").unwrap();
	let mut cloned = html_el.clone_node();
	cloned.remove_whitespace();
	assert_eq!(
		cloned.outer_html(),
		"<html lang=\"en\"><meta charset=\"UTF-8\"><title>test</title><p>test</p></html>"
	);
}
