use node_html_parser::{parse_with_options, Options};
use std::fs;

#[test]
fn issue_260_parse_none_closed_tags_and_specific_button() {
    // 读取与 JS 测试同名资源
    let path = "tests/assets/html/a1supplements.com.html"; // 资源路径 (与 js/tests/assets/html 下对应)
    let html = fs::read_to_string(path).expect("read html asset");
    let mut opts = Options::default();
    opts.parse_none_closed_tags = true;
    let root = parse_with_options(&html, &opts);
    let btn = root.query_selector("#sca-fg-today-offer-widget").expect("button exists");
    assert_eq!(btn.outer_html(), "<button id=\"sca-fg-today-offer-widget\"></button>");
}
