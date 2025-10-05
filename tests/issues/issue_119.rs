use node_html_parser::parse;

// Port of js/tests/issues/119.js (closest())
#[test]
fn issue_119_closest_class_chain() {
	let html = "<div class=\"a b\">\n\t<div id=\"el\">\n\t\t<div class=\"a b\">foo</div>\n\t</div>\n</div>";
	let root = parse(html);
	let el = root.query_selector("#el").unwrap();
	let d = el.closest(".a.b").unwrap();
	assert_eq!(d.to_string(), html);
}

#[test]
fn issue_119_closest_attribute_query() {
	let html = "<ul item=\"111\" id=\"list\"><li>Hello World<ul item=\"222\"></ul></li></ul>";
	let root = parse(html);
	let li = root.query_selector("li").unwrap();
	let ul = li.closest("ul[item]").unwrap();
	assert_eq!(ul.get_attr("item"), Some("111"));
}

#[test]
fn issue_119_various_closest_chain() {
	let html = "<div class=\"f\"><span><div id=\"foo\"></div></span></div>";
	let root = parse(html);
	let d = root.query_selector("#foo").unwrap();
	assert_eq!(
		d.closest("div").unwrap().to_string(),
		"<div id=\"foo\"></div>"
	);
	assert_eq!(
		d.closest("span").unwrap().to_string(),
		"<span><div id=\"foo\"></div></span>"
	);
	assert_eq!(d.closest("div.f").unwrap().to_string(), html);
}

#[test]
fn issue_119_nested_case_84_subset() {
	let root = parse("<a id=\"id\" data-id=\"myid\">\n\t<div>\n\t\t<span class=\"a b\"></span>\n\t\t<span data-bar=\"bar\">\n\t\t\t<div id=\"foo\">\n\t\t\t\t<a id=\"id\" data-id=\"myid\"></a>\n\t\t\t</div>\n\t\t</span>\n\t\t<span data-bar=\"foo\"></span>\n\t</div>\n</a>");
	let div = root.query_selector("#foo").unwrap();
	assert!(std::ptr::eq(
		div.closest("#id").unwrap(),
		root.first_element_child().unwrap()
	));
	assert!(div.closest("span.a").is_none());
	assert!(div.closest("span.b").is_none());
	assert!(div.closest("span.a.b").is_none());
	assert_eq!(
		div.closest("span").unwrap().get_attr("data-bar"),
		Some("bar")
	);
	assert_eq!(
		div.closest("[data-bar]").unwrap().get_attr("data-bar"),
		Some("bar")
	);
	assert_eq!(
		div.closest("[data-bar=\"bar\"]")
			.unwrap()
			.get_attr("data-bar"),
		Some("bar")
	);
	assert!(div.closest("[data-bar=\"foo\"]").is_none());
	assert!(std::ptr::eq(
		div.closest("[data-id=myid]").unwrap(),
		root.first_element_child().unwrap()
	));
	assert!(std::ptr::eq(
		div.closest("[data-id=\"myid\"]").unwrap(),
		root.first_element_child().unwrap()
	));
}
