use node_html_parser::{parse_with_options, Node, Options};

fn as_element<'a>(n: &'a Node) -> &'a node_html_parser::HTMLElement {
	match n {
		Node::Element(e) => e,
		_ => panic!("expected element"),
	}
}
fn as_text<'a>(n: &'a Node) -> &'a node_html_parser::TextNode {
	match n {
		Node::Text(t) => t,
		_ => panic!("expected text"),
	}
}

#[test]
fn nested_a_preserved_by_default() {
	let html = "<A href=\"#\"><b>link <a href=\"#\">nested link</a> end</b></A>";
	let root = parse_with_options(html, &Options::default());
	// Inner HTML of synthetic root should equal original (serialization normalizations allowed for voids only)
	assert_eq!(root.inner_html(), html);
	assert_eq!(root.children.len(), 1);
	let a1 = as_element(&root.children[0]);
	assert_eq!(a1.name(), "A");
	// a1 node type already known by pattern (Element). Keep a symbolic assert via its serialization if needed.
	assert_eq!(a1.children.len(), 1);
	let b = as_element(&a1.children[0]);
	assert_eq!(b.name(), "b");
	assert_eq!(b.children.len(), 3);
	// b.text() should concatenate
	assert_eq!(b.text(), "link nested link end");
	let a2 = as_element(&b.children[1]);
	assert_eq!(a2.name().to_ascii_lowercase(), "a");
	// inner a element likewise confirmed by variant matching
	assert_eq!(a2.children.len(), 1);
	if let Node::Text(t) = &a2.children[0] {
		assert_eq!(t.raw, "nested link");
	} else {
		panic!("expected text child of inner a");
	}
	let end_text = as_text(&b.children[2]);
	assert_eq!(end_text.text(), " end");
}

#[test]
fn nested_a_fixed_with_option() {
	let html = "<A href=\"#\"><b>link <a href=\"#\">nested link</a> end</b></A>";
	let mut opts = Options::default();
	opts.fix_nested_a_tags = true;
	let root = parse_with_options(html, &opts);
	// Expected JS parity output
	let expected = "<A href=\"#\"><b>link </b></A><a href=\"#\">nested link</a> end";
	assert_eq!(
		root.inner_html(),
		expected,
		"fixNestedATags serialization mismatch: {}",
		root.inner_html()
	);
	assert_eq!(root.children.len(), 3);
	let a1 = as_element(&root.children[0]);
	assert_eq!(a1.name(), "A");
	let b = as_element(&a1.children[0]);
	assert_eq!(b.text(), "link ");
	let a2 = as_element(&root.children[1]);
	assert_eq!(a2.name().to_ascii_lowercase(), "a");
	if let Node::Text(t) = &a2.children[0] {
		assert_eq!(t.raw, "nested link");
	} else {
		panic!("nested a child text");
	}
	let tail = as_text(&root.children[2]);
	assert_eq!(tail.text(), " end");
}
