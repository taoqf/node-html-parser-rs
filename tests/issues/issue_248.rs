use node_html_parser::{valid, Options};

#[test]
fn issue_248_custom_void_tag_validation() {
	let mut opts = Options::default();
	opts.void_tag.tags = Some(vec!["x-tag".into()]);
	assert!(valid("<div><x-tag></div>", &opts));
}

#[test]
fn issue_248_custom_void_tag_selfclosed_validation() {
	let mut opts = Options::default();
	opts.void_tag.tags = Some(vec!["x-tag".into()]);
	opts.void_tag.add_closing_slash = true;
	assert!(valid("<div><x-tag /></div>", &opts));
}
