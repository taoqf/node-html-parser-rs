use node_html_parser::{parse_with_options, Options};

#[test]
fn issue_200_angular_two_way_binding_attribute() {
	let html = "<input [(ngModel)]=\"a\" />"; // In JS test, attribute should be preserved exactly
	let mut opts = Options::default();
	opts.void_tag.add_closing_slash = true; // mirror JS voidTag.closingSlash
	let mut root = parse_with_options(html, &opts);
	assert_eq!(root.to_string(), "<input [(ngModel)]=\"a\" />");
	// Manually get first element child mutably (the input)
	let input_el = root.first_element_child_mut().expect("input present");
	assert_eq!(input_el.get_attribute("[(ngModel)]"), Some("a".into()));
}
