use node_html_parser::parse;

#[test]
fn issue_84_query_selector_variations() {
    let root = parse("<a id=\"id\" data-id=\"myid\"><div><span class=\"a b\"></span><span></span><span></span></div></a>");
    let a = root.query_selector("a").unwrap();
    assert!(std::ptr::eq(root.query_selector("#id").unwrap(), a));
    let span_ab = a.query_selector("span.a").unwrap();
    assert!(std::ptr::eq(root.query_selector("span.a").unwrap(), span_ab));
    assert!(std::ptr::eq(root.query_selector("span.b").unwrap(), span_ab));
    assert!(std::ptr::eq(root.query_selector("span.a.b").unwrap(), span_ab));
    assert!(std::ptr::eq(root.query_selector("#id .b").unwrap(), span_ab));
    assert!(std::ptr::eq(root.query_selector("#id span").unwrap(), span_ab));
    assert!(std::ptr::eq(root.query_selector("[data-id=myid]").unwrap(), a));
    assert!(std::ptr::eq(root.query_selector("[data-id=\"myid\"]").unwrap(), a));
}

#[test]
fn issue_84_query_selector_all_variations() {
    let root = parse("<a id=\"id\"><div><span class=\"a b\"></span><span></span><span></span></div></a>");
    let a = root.query_selector("a").unwrap();
    let first_span = root.query_selector("span.a").unwrap();
    // 简单对比长度与指针身份（顺序）
    assert_eq!(root.query_selector_all("#id").len(), 1);
    assert!(std::ptr::eq(root.query_selector_all("#id")[0], a));
    assert!(std::ptr::eq(root.query_selector_all("span.a")[0], first_span));
    assert!(std::ptr::eq(root.query_selector_all("span.b")[0], first_span));
    assert!(std::ptr::eq(root.query_selector_all("span.a.b")[0], first_span));
    assert!(std::ptr::eq(root.query_selector_all("#id .b")[0], first_span));
    // #id span -> 3 spans
    assert_eq!(root.query_selector_all("#id span").len(), 3);
    // #id, #id .b -> 两个元素：a 与 第一个 span
    let combo = root.query_selector_all("#id, #id .b");
    assert_eq!(combo.len(), 2);
    assert!(std::ptr::eq(combo[0], a));
    assert!(std::ptr::eq(combo[1], first_span));

    // time/.date (去重) 情况
    let root2 = parse("<time class=\"date\">");
    let list = root2.query_selector_all("time,.date");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name(), "time");
}
