use node_html_parser::{parse_with_options, Options};

#[test]
fn raw_attributes_escape_preserved() {
    let mut root = parse_with_options("<p a=12 data-id=\"!$$&amp;\" yAz='1' @click=\"doSmt()\"></p>", &Options::default());
    let p = root.first_element_child_mut().unwrap();
    let raw = p.raw_attributes().clone();
    assert_eq!(raw.get("a"), Some(&"12".to_string()));
    assert_eq!(raw.get("data-id"), Some(&"!$$&amp;".to_string()));
    assert_eq!(raw.get("yAz"), Some(&"1".to_string()));
    assert_eq!(raw.get("@click"), Some(&"doSmt()".to_string()));
}

#[test]
fn attributes_decoded_and_boolean() {
    let mut root = parse_with_options("<p a=12 data-id=\"!$$&amp;\" yAz='1' class=\"\" disabled></p>", &Options::default());
    let p = root.first_element_child_mut().unwrap();
    let attrs = p.attributes().clone();
    assert_eq!(attrs.get("a"), Some(&"12".to_string()));
    assert_eq!(attrs.get("data-id"), Some(&"!$$&".to_string()));
    assert_eq!(attrs.get("yAz"), Some(&"1".to_string()));
    assert_eq!(attrs.get("disabled"), Some(&"".to_string()));
}

#[test]
fn get_set_attribute_behaviors() {
    let mut root = parse_with_options("<p a=12></p>", &Options::default());
    let p = root.first_element_child_mut().unwrap();
    assert_eq!(p.get_attribute("a"), Some("12".into()));
    p.set_attribute("a", "13");
    assert_eq!(p.get_attribute("a"), Some("13".into()));
    p.set_attribute("required", "");
    let ser = p.outer_html();
    assert!(ser.contains("required"));
}

#[test]
fn set_multiple_attributes_overwrite() {
    let mut root = parse_with_options("<p a=12 data-id=\"x\" class=\"\" disabled></p>", &Options::default());
    let p = root.first_element_child_mut().unwrap();
    p.set_attributes(&[("c".into(), "12".into()), ("d".into(), "&&<>foo".into())]);
    assert_eq!(p.get_attribute("c"), Some("12".into()));
    assert_eq!(p.get_attribute("d"), Some("&&<>foo".into()));
    let ser = p.outer_html();
    assert_eq!(ser, "<p c=\"12\" d=\"&&<>foo\"></p>");
}

#[test]
fn remove_and_has_attribute() {
    let mut root = parse_with_options("<input required>", &Options::default());
    let input = root.first_element_child_mut().unwrap();
    assert!(input.has_attribute("required"));
    input.remove_attribute("required");
    assert!(!input.has_attribute("required"));
}
