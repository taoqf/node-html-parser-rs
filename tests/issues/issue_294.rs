use node_html_parser::{parse_with_options, valid, Options};

#[test]
fn issue_294_closing_tag_missing_parses_and_valid() {
    let content = "<body><main class=h-entry><p>hello</main></body>";
    // validator should consider this valid (JS parity)
    assert!(valid(content, &Options::default()));
    let root = parse_with_options(content, &Options::default());
    assert_eq!(root.to_string(), "<body><main class=h-entry><p>hello</p></main></body>");
    let list = root.query_selector_all(".h-entry");
    assert_eq!(list.len(), 1);
}
