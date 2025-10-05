use node_html_parser::{parse_with_options, Options, HTMLElement};

#[test]
fn insert_adjacent_basic_positions() {
    let mut root = parse_with_options("<div id='r'><p id='a'></p></div>", &Options::default());
    let r = root.first_element_child_mut().unwrap();
    r.insert_adjacent_html("beforeend", "<span id='b'></span>").unwrap();
    r.insert_adjacent_html("afterbegin", "<em id='c'></em>").unwrap();
    let ids: Vec<String> = r.children_elements().iter().map(|e| e.get_attr("id").unwrap().into()).collect();
    assert_eq!(ids, vec!["c", "a", "b"]);
}

#[test]
fn mutation_before_after_replace_remove_sequence() {
    let mut root = parse_with_options("<div><p id='a'></p><span id='b'></span></div>", &Options::default());
    let div = root.first_element_child_mut().unwrap();
    {
        let first = div.first_element_child_mut().unwrap();
        first.after("<em id='x'></em>");
        first.before("<strong id='y'></strong>");
    }
    // replace a
    let idx_a = div.children_elements().iter().position(|e| e.get_attr("id") == Some("a")).unwrap();
    let raw_ptr: *mut HTMLElement = match &mut div.children[idx_a] { node_html_parser::Node::Element(bx) => &mut **bx, _ => panic!() };
    unsafe { (*raw_ptr).replace_with("<u id='ra'></u>"); }
    let ids1: Vec<String> = div.children_elements().iter().map(|e| e.get_attr("id").unwrap().into()).collect();
    assert!(ids1.contains(&"ra".into()));
}
