use node_html_parser::{parse_with_options, Options};

#[test]
fn text_content_no_entity_encoding() {
	let root = parse_with_options("<div id='x'></div>", &Options::default());
	let el = root.query_selector("div").unwrap().clone();
	let mut cloned = el.clone();
	cloned.set_text_content("<span>&amp;</span>");
	// textContent should not encode entities
	assert!(
		cloned.inner_html().contains("<span>&amp;</span>"),
		"textContent setter should keep raw text"
	);
}

#[test]
fn inner_html_empty_fallback_textnode() {
	let root = parse_with_options("<div id='x'></div>", &Options::default());
	let el = root.query_selector("div").unwrap().clone();
	let mut cloned = el.clone();
	cloned.set_inner_html("");
	// JS behavior: Should empty innerHTML result in empty childNodes or an empty TextNode? JS parses empty string to fragment -> childNodes length 0, but implementation fallback -> TextNode
	assert_eq!(cloned.inner_html(), "");
	assert!(
		matches!(cloned.first_child(), Some(node_html_parser::Node::Text(_))),
		"Expect a TextNode fallback"
	);
}
