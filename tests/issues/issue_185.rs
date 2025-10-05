use node_html_parser::{parse_with_options, Node, Options};

#[test]
fn issue_185_previous_sibling() {
	let root = parse_with_options("<div>ccc<a></a><b></b></div>", &Options::default());
	let a = root.query_selector("a").unwrap();
	let b = root.query_selector("b").unwrap();
	// previousSibling: 在我们的实现中 previous_sibling() 返回 Option<&Node>
	let prev_b = b.previous_sibling().unwrap();
	match prev_b {
		Node::Element(e) => assert!(std::ptr::eq(&**e as *const _, a as *const _)),
		_ => panic!("expected element"),
	}
	// a.previousSibling 应该是文本节点 "ccc" (JS 中 not null)
	assert!(a.previous_sibling().is_some());
}

#[test]
fn issue_185_previous_element_sibling() {
	let root = parse_with_options("<div>ccc<a></a><b></b></div>", &Options::default());
	let a = root.query_selector("a").unwrap();
	let b = root.query_selector("b").unwrap();
	let prev_el = b.previous_element_sibling().unwrap();
	assert!(std::ptr::eq(prev_el as *const _, a as *const _));
	assert!(a.previous_element_sibling().is_none());
}
