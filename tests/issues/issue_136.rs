use node_html_parser::parse;

// Port of js/tests/issues/136.js (attribute containing HTML-like text)
#[test]
fn issue_136_attribute_with_embedded_tags_preserved() {
	let html = "<a testArg=\"<a>test</a>\" secondArg=\"some_thing\">some test content</a>";
	let root = parse(html);
	let el = root.first_element_child().unwrap();
	assert_eq!(el.get_attr("testArg"), Some("<a>test</a>"));
	assert_eq!(el.get_attr("secondArg"), Some("some_thing"));
	assert_eq!(el.text_content(), "some test content");
	assert_eq!(
		el.raw_attrs_str().trim(),
		"testArg=\"<a>test</a>\" secondArg=\"some_thing\""
	);
	let ser = root.to_string();
	assert_eq!(
		ser, html,
		"serialization mismatch after balanced attr parse repair: {}",
		ser
	);
	assert_eq!(el.name(), "a");
}
