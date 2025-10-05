use node_html_parser::parse;

// Port of js/tests/issues/129.js (prototype pollution protection)
#[test]
fn issue_129_prevent_prototype_pollution() {
	let root = parse("<a href=\"#\" __proto__=\"polluted=true\">");
	let el = root.first_element_child().unwrap();
	// clone to obtain a mutable Element for attribute API (attributes() needs &mut self)
	let mut cloned = el.clone_shallow();
	let attrs_map = cloned.attributes();
	assert!(
		!attrs_map.contains_key("polluted"),
		"prototype polluted via attributes map"
	);
	let raw = cloned.raw_attributes();
	// Accept either '__proto__' captured or silently ignored, but never create 'polluted'
	if !raw.contains_key("__proto__") {
		// ensure still no polluted key
		assert!(!raw.contains_key("polluted"));
	}
}
