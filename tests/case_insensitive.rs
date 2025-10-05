use node_html_parser::{parse_with_options, Options};

#[test]
fn class_case_insensitive_selector() {
	let root = parse_with_options(
		"<dIv CLASS=\"a\" data-KEY=\"val\"></DiV>",
		&Options::default(),
	);
	assert_eq!(root.query_selector_all(".a").len(), 1);
}

#[test]
fn tag_case_insensitive_selector() {
	let root = parse_with_options(
		"<dIv CLASS=\"a\" data-KEY=\"val\"></DiV>",
		&Options::default(),
	);
	assert_eq!(root.query_selector_all("div").len(), 1);
}

#[test]
fn attribute_case_insensitive_selector() {
	let root = parse_with_options(
		"<dIv CLASS=\"a\" data-KEY=\"val\"></DiV>",
		&Options::default(),
	);
	assert_eq!(root.query_selector_all("[data-key=\"val\"]").len(), 1);
}
