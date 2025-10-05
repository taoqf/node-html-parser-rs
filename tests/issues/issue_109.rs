use node_html_parser::{parse_with_options, Options};

#[test]
fn issue_109_textarea_not_self_close() {
	let html = "<textarea id=\"input_3\"></textarea>";
	let root = parse_with_options(html, &Options::default());
	assert_eq!(root.first_element_child().unwrap().to_string(), html);
}

#[test]
fn issue_109_input_self_closing_space_normalized() {
	let html = "<input value=\"foo\" />"; // original source with space+slash
	let root = parse_with_options(html, &Options::default());
	let out = root.first_element_child().unwrap().to_string();
	assert!(
		matches!(
			out.as_str(),
			"<input value=\"foo\">" | "<input value=\"foo\" >" | "<input value=\"foo\" />"
		),
		"unexpected serialization: {}",
		out
	);
}
