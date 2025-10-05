use node_html_parser::{parse_with_options, Options};

// 注释解析与相关 API
#[test]
fn comments_default_not_included() {
	let root = parse_with_options("<div><a><!-- c --></a></div>", &Options::default());
	let ser = root.outer_html();
	assert_eq!(ser, "<div><a></a></div>");
}

#[test]
fn comments_included_when_option_enabled() {
	let mut opts = Options::default();
	opts.comment = true;
	let root = parse_with_options("<div><a><!-- my comment --></a></div>", &opts);
	assert!(root.outer_html().contains("<!-- my comment -->"));
}

#[test]
fn comments_not_parsed_inside_content_disabled() {
	let mut opts = Options::default();
	opts.comment = true;
	let root = parse_with_options("<div><!--<a></a>--></div>", &opts);
	assert!(root.outer_html().contains("<!--<a></a>-->"));
}
