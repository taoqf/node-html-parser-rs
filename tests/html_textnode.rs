use node_html_parser::{parse_with_options, Options, Node, TextNode};

#[test]
fn textnode_is_whitespace_variants() {
    let n1 = TextNode::new("".into());
    assert!(n1.is_whitespace());
    let n2 = TextNode::new(" \t".into());
    assert!(n2.is_whitespace());
    let n3 = TextNode::new(" \t&nbsp; \t".into());
    assert!(n3.is_whitespace());
}

#[test]
fn parse_plain_text_document() {
    let root = parse_with_options("hello mmstudio", &Options::default());
    match root.first_child().unwrap() { Node::Text(t) => assert_eq!(t.raw, "hello mmstudio"), _ => panic!("expected text") }
    assert_eq!(root.to_string(), "hello mmstudio");
}
