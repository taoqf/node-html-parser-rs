use node_html_parser::{parse_with_options, Node, Options};

fn get_first_child(node: &node_html_parser::HTMLElement) -> &Node {
	node.children.first().expect("first child")
}

#[test]
fn clone_html_element() {
	let html = "<div foo><bar>text</bar></div>";
	let root = parse_with_options(html, &Options::default());
	let div_el = match get_first_child(&root) {
		Node::Element(e) => e,
		_ => panic!("expected element"),
	};
	let cloned = div_el.clone();
	assert_eq!(cloned.to_string(), "<div foo><bar>text</bar></div>");
}

#[test]
fn clone_text_node() {
	let html = "<div foo>txt</div>";
	let root = parse_with_options(html, &Options::default());
	let div_el = match get_first_child(&root) {
		Node::Element(e) => e,
		_ => panic!("expected element"),
	};
	let txt_node = match div_el.children.first().unwrap() {
		Node::Text(t) => t.clone(),
		_ => panic!("expected text"),
	};
	assert_eq!(txt_node.raw, "txt");
	// emulate JS clone() semantics: TextNode implements Clone, so direct clone
	let cloned = txt_node.clone();
	assert_eq!(cloned.raw, "txt");
	assert_eq!(cloned.range(), txt_node.range());
}

#[test]
fn clone_comment_node() {
	// JS test uses '<!== comment ==>' style; adapt: we need real HTML comment.
	let html = "<div foo><!-- comment --></div>";
	let mut opts = Options::default();
	opts.comment = true;
	let root = parse_with_options(html, &opts);
	let div_el = match get_first_child(&root) {
		Node::Element(e) => e,
		_ => panic!("expected element"),
	};
	let comment = match div_el.children.first().unwrap() {
		Node::Comment(c) => c.clone(),
		other => panic!("expected comment got {:?}", other),
	};
	assert_eq!(comment.text, " comment ");
	let cloned = comment.clone();
	assert_eq!(cloned.text, comment.text);
	assert_eq!(cloned.range(), comment.range());
}
