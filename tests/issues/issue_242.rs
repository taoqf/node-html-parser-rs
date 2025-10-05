use node_html_parser::{parse, Options, Node};

#[test]
fn issue_242_raw_attributes_and_get_attribute() {
    let mut root = parse("<div><a href=\"/\" rel=\"home\">Git Hub</a></div>");
    // 手动DFS获取可变 a
    fn find_a<'a>(el: &'a mut node_html_parser::HTMLElement) -> Option<&'a mut node_html_parser::HTMLElement> {
        for child in el.children.iter_mut() {
            if let Node::Element(e) = child { if e.name()=="a" { return Some(e); } if let Some(f)=find_a(e){return Some(f);} }
        }
        None
    }
    let root_el = root.first_element_child_mut().unwrap();
    let a_mut = find_a(root_el).unwrap();
    assert!(a_mut.raw_attrs_str().contains("href=\"/\""));
    assert_eq!(a_mut.get_attribute("href"), Some("/".into()));
}

#[test]
fn issue_242_get_code_when_pre_not_block() {
    // blockTextElements 空 -> pre 内部解析
    let mut opts = Options::default();
    opts.block_text_elements.clear();
    let root = node_html_parser::parse_with_options("<pre>\n  <code>test</code>\n</pre>", &opts);
    let code = root.get_elements_by_tag_name("code");
    assert_eq!(code.len(), 1);
    assert_eq!(code[0].text(), "test");
}

#[test]
fn issue_242_block_text_element_toggle() {
    let html = "sample <b><strong>text</strong> inside tags</b> <script>text inside script</script>";
    // script not blocked
    let mut opts1 = Options::default();
    opts1.block_text_elements.insert("script".into(), false);
    let root1 = node_html_parser::parse_with_options(html, &opts1);
    assert_eq!(root1.text(), "sample text inside tags ");
    // script blocked: text included
    let mut opts2 = Options::default();
    opts2.block_text_elements.insert("script".into(), true);
    let root2 = node_html_parser::parse_with_options(html, &opts2);
    assert_eq!(root2.text(), "sample text inside tags text inside script");
}
