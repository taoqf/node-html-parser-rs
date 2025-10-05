use node_html_parser::parse;

#[test]
fn issue_48_decoded_text_numeric_entity() {
	let root = parse("<div>The king&#39;s hat is on fire!</div>");
	let div = root.query_selector("div").unwrap();
	assert_eq!(div.text(), "The king's hat is on fire!");
}

#[test]
fn issue_48_decoded_text_named_entity() {
	let root = parse("<div>The king&apos;s hat is on fire!</div>");
	let div = root.query_selector("div").unwrap();
	assert_eq!(div.text(), "The king's hat is on fire!");
}
