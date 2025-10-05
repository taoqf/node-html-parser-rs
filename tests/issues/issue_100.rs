use node_html_parser::{parse_with_options, Options};

#[test]
fn issue_100_basic_query_selector_all() {
	let html = "<div>foo<div class=\"a b\" id=\"a\">bar</div></div>";
	let root = parse_with_options(html, &Options::default());
	assert_eq!(root.query_selector_all("a").len(), 0);
	assert!(root.query_selector("a").is_none());
	assert!(root.query_selector("#b").is_none());
	assert_eq!(root.query_selector_all(".a").len(), 1);
	assert_eq!(root.query_selector_all(".b").len(), 1);
	assert!(root.query_selector(".a").is_some());
	let ab = root.query_selector_all(".a,.b");
	assert_eq!(ab.len(), 1);
	let div = ab[0];
	assert!(std::ptr::eq(div, root.query_selector(".a,.b").unwrap()));
	assert!(std::ptr::eq(div, root.query_selector("#a").unwrap()));
}
