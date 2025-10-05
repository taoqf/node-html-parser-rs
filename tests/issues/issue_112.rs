use node_html_parser::HTMLElement;

// Port of js/tests/issues/112.js
#[test]
fn issue_112_html_element_id_ctor_and_attr_sync() {
	// new HTMLElement('div', {}, '', null) in JS corresponds roughly to:
	// tag = Some("div"), raw_attrs = "", attrs = vec![], is_void=false, void_add_slash=false
	let el = HTMLElement::new(Some("div".to_string()), String::new(), vec![], false, false);
	assert_eq!(el.id, "");
	// getAttribute('id') -> None
	// In Rust version, get_attr returns Option<&str>; ensure it's None
	assert!(el.get_attr("id").is_none());
	assert_eq!(el.to_string(), "<div></div>");

	// With id provided
	let el2 = HTMLElement::new(
		Some("div".to_string()),
		"id=\"id\"".to_string(),
		vec![("id".to_string(), "id".to_string())],
		false,
		false,
	);
	assert_eq!(el2.id, "id");
	assert_eq!(el2.get_attr("id"), Some("id"));
	assert_eq!(el2.to_string(), "<div id=\"id\"></div>");
}

#[test]
fn issue_112_remove_attribute_updates_id() {
	let html = "<div id=\"id\"></div>";
	let root = node_html_parser::parse(html);
	let el = root.first_element_child().unwrap();
	assert_eq!(el.id, "id");
	assert_eq!(el.get_attr("id"), Some("id"));
	// removeAttribute
	// clone, mutate on clone, since original el is behind immutable reference
	let mut cloned = el.clone_shallow();
	// ensure id removal path works
	cloned.remove_id();
	assert_eq!(cloned.id, "");
	assert!(cloned.get_attr("id").is_none());
	// after removing id, serialization should drop id
	assert_eq!(cloned.to_string(), "<div></div>");
}

#[test]
fn issue_112_set_attribute_updates_id() {
	let html = "<div></div>";
	let root = node_html_parser::parse(html);
	let el = root.first_element_child().unwrap();
	assert_eq!(el.id, "");
	assert!(el.get_attr("id").is_none());
	let mut cloned = el.clone_shallow();
	cloned.set_id("id");
	assert_eq!(cloned.id, "id");
	assert_eq!(cloned.get_attr("id"), Some("id"));
	assert_eq!(cloned.to_string(), "<div id=\"id\"></div>");
}
