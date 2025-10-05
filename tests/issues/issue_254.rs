use node_html_parser::parse;

#[test]
fn issue_254_abbr_should_not_be_a_newline() {
    let root = parse("<div>Hello <abbr>World</abbr></div>");
    let div = root.query_selector("div").unwrap();
    // 预期: "Hello World" (abbr 不应触发换行)
    assert_eq!(div.text().replace('\n', " ").trim(), "Hello World");
}
