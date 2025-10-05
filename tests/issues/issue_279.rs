use node_html_parser::{parse, valid};
use std::fs;

#[test]
fn issue_279_large_html_valid_and_query_count() {
    let path = "tests/assets/html/beego package - github.com_astaxie_beego - Go Packages.html";
    let content = fs::read_to_string(path).expect("read asset");
    assert!(valid(&content, &node_html_parser::Options::default()));
    let root = parse(&content);
    let list = root.query_selector_all(".go-Footer-listItem");
    assert_eq!(list.len(), 6);
}
