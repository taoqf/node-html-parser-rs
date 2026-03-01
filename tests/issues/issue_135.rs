use node_html_parser::parse;

// Port of js/tests/issues/135.js (text decoding vs innerHTML)
#[test]
fn issue_135_not_decode_inner_html() {
	let content = "&lt;p&gt; Not a p tag &lt;br /&gt; at all";
	let html = format!("<div>{}</div>", content);
	let root = parse(&html);
	let div = root.first_element_child().unwrap();
	assert_eq!(div.inner_html(), content); // innerHTML stays encoded
	assert_eq!(div.text_content(), "<p> Not a p tag <br /> at all"); // textContent decoded
}

#[test]
fn issue_135_textnode_raw_text_preserved() {
	let content = "&lt;p&gt; Not a p tag &lt;br /&gt; at all";
	let root = parse(&format!("<div>{}</div>", content));
	let div = root.first_element_child().unwrap();
	let first_child = div.children.get(0).expect("text node expected");
	if let node_html_parser::Node::Text(t) = first_child {
		assert_eq!(t.raw_text(), content);
	} else {
		panic!("expected text node");
	}
}

#[test]
fn issue_135_decode_text_property() {
	let encoded = "My&gt;text";
	let root = parse(&format!("<p>{}</p>", encoded));
	let p = root.first_element_child().unwrap();
	assert_eq!(p.inner_html(), encoded);
	assert_eq!(p.text_content(), "My>text");
	let tn = p.children.get(0).unwrap();
	if let node_html_parser::Node::Text(t) = tn {
		assert_eq!(t.decoded_text(), "My>text");
	} else {
		panic!("expected text node");
	}
}
