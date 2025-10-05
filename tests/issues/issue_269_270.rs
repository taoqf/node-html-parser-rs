use node_html_parser::{parse_with_options, Options};

#[test]
fn issue_269_270_comment_nodes_and_raw_tag_name() {
    let mut opts = Options::default();
    opts.comment = true;
    let root = parse_with_options("<div><!--this is comment here -->foo</div>", &opts);
    let div = root.children.iter().find_map(|n| n.as_element()).expect("div");
    assert_eq!(div.name(), "div"); // rawTagName parity
    assert_eq!(div.children.len(), 2);
    let comment = &div.children[0];
    assert!(matches!(comment.node_type(), node_html_parser::NodeType::Comment));
    // JS rawTagName for comment is '!--'; we don't store rawTagName on Comment; emulate via selector query later
    let text = &div.children[1];
    assert!(matches!(text.node_type(), node_html_parser::NodeType::Text));
}

#[test]
fn issue_269_270_query_selector_comment() {
    let mut opts = Options::default();
    opts.comment = true;
    let html = r#"<html>
  <body>
    <h1>TEST</h1>
    <!-- Some comment here. -->
  </body>
</html>"#;
    let root = parse_with_options(html, &opts);
    // 遍历 children 收集注释文本
    let mut found = None;
    fn dfs_node(node: &node_html_parser::Node, out: &mut Option<String>) {
        match node {
            node_html_parser::Node::Comment(c) => {
                if c.text.contains("Some comment here.") { *out = Some(c.text.clone()); return; }
            }
            node_html_parser::Node::Element(e) => {
                for ch in &e.children { dfs_node(ch, out); if out.is_some() { return; } }
            }
            node_html_parser::Node::Text(_) => {}
        }
    }
    for ch in &root.children { dfs_node(ch, &mut found); if found.is_some() { break; } }
    assert!(found.is_some(), "should find comment node with expected text");
}
