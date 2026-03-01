use node_html_parser::{parse_with_options, Options};

// issue 176: 修改 tagName (set_tag_name)
#[test]
fn issue_176_change_tag_name() {
	let html = "<foo></foo>";
	let mut root = parse_with_options(html, &Options::default());
	let el = root.first_element_child_mut().unwrap();
	el.set_tag_name("bar");
	assert_eq!(root.to_string(), "<bar></bar>");
}
#[test]
fn issue_176_change_tag_name_uppercase_input() {
	let html = "<foo></foo>";
	let mut root = parse_with_options(html, &Options::default());
	let el = root.first_element_child_mut().unwrap();
	el.set_tag_name("BAR");
	assert_eq!(root.to_string(), "<bar></bar>");
}
