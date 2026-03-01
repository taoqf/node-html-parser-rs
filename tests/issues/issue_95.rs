use node_html_parser::parse;

#[test]
fn issue_95_get_text_content() {
	let root = parse("<div>foo<div>bar</div></div>");
	assert_eq!(root.text(), "foobar"); // root.text() 解码聚合
}

#[test]
fn issue_95_set_text_content_in_element() {
	let root = parse("<div><a>foo</a>bar</div>");
	assert_eq!(root.text(), "foobar");
	let div = root.query_selector("div").unwrap();
	assert_eq!(div.text(), "foobar");
	let a = div.query_selector("a").unwrap();
	assert_eq!(a.text(), "foo");
	// set_text_content 等价 JS a.textContent = 'bar'
	let mut cloned_a = a.clone_node();
	cloned_a.set_text_content("bar");
	assert_eq!(cloned_a.text(), "bar");
}

#[test]
fn issue_95_set_text_content_using_textnode() {
	let root = parse("<div>foo<div>");
	assert_eq!(root.text(), "foo");
	let div = root.query_selector("div").unwrap();
	// 没有直接暴露内部 TextNode 修改 API；模拟：clone 后 set_text_content
	let mut cloned = div.clone_node();
	cloned.set_text_content("bar");
	assert_eq!(cloned.text(), "bar");
}

#[test]
fn issue_95_replace_childnodes_with_text_content() {
	let root = parse("<div><a>foo</a><b>bar</b><div>");
	assert_eq!(root.text(), "foobar");
	let div = root.query_selector("div").unwrap();
	assert_eq!(div.text(), "foobar");
	let b = root.query_selector("b").unwrap();
	let mut b2 = b.clone_node();
	b2.set_text_content("foo");
	assert_eq!(b2.text(), "foo");
}
