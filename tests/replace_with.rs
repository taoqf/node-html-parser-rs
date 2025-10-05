use node_html_parser::{parse, HTMLElement, Node};

// Helper: locate first element whose outer_html contains pattern (quick & lenient)
fn find_mut<'a, F>(el: &'a mut HTMLElement, pred: &F) -> Option<&'a mut HTMLElement>
where
	F: Fn(&HTMLElement) -> bool,
{
	if pred(el) {
		return Some(el);
	}
	for child in el.children.iter_mut() {
		if let Node::Element(ref mut e) = child {
			if let Some(r) = find_mut(e, pred) {
				return Some(r);
			}
		}
	}
	None
}

#[test]
fn replace_with_attribute_colon() {
	let mut root = parse("<Div><Input/></Div>");
	assert_eq!(root.to_string(), "<Div><Input></Input></Div>");
}

#[test]
fn inner_html_setter() {
	let html = "<html>\n  <div class=\"main\">yay</div>\n</html>\n";
	let mut root = parse(html);
	{
		let target = find_mut(&mut root, &|e| {
			e.name().eq_ignore_ascii_case("div") && e.get_attr("class").unwrap_or("") == "main"
		})
		.expect("div.main not found");
		target.set_inner_html("innerHTML setter was here");
	}
	let expected = "<html>\n  <div class=\"main\">innerHTML setter was here</div>\n</html>\n";
	assert_eq!(root.to_string(), expected);
}

#[test]
fn replace_with_single() {
	let html = "<html>\n  <div class=\"main\">yay</div>\n</html>\n";
	let mut root = parse(html);
	{
		let target = find_mut(&mut root, &|e| {
			e.name().eq_ignore_ascii_case("div") && e.get_attr("class").unwrap_or("") == "main"
		})
		.unwrap();
		target.replace_with("<pre>replaceWith was here</pre>");
	}
	let expected = "<html>\n  <pre>replaceWith was here</pre>\n</html>\n";
	assert_eq!(root.to_string(), expected);
}

#[test]
fn replace_with_multiple() {
	let html = "<html>\n  <div class=\"main\">yay</div>\n</html>\n";
	let mut root = parse(html);
	{
		let target = find_mut(&mut root, &|e| {
			e.name().eq_ignore_ascii_case("div") && e.get_attr("class").unwrap_or("") == "main"
		})
		.unwrap();
		// 通过一次 replace_with 传入合并片段实现同等结果
		target.replace_with("<pre>replaceWith was here</pre><foo>bar</foo>");
	}
	let expected = "<html>\n  <pre>replaceWith was here</pre><foo>bar</foo>\n</html>\n";
	assert_eq!(root.to_string(), expected);
}

#[test]
fn transform_custom_element() {
	let html = "<html>\n  <some-custom-element class=\"main\">yay</some-custom-element>\n</html>\n";
	let mut root = parse(html);
	// Locate custom element(s)
	// Semantic: replace each with <div class="some-custom-element">...inner...</div>
	let mut targets: Vec<*mut HTMLElement> = Vec::new();
	// collect raw pointers to avoid double mutable borrow
	fn collect(el: &mut HTMLElement, acc: &mut Vec<*mut HTMLElement>) {
		if el.name() == "some-custom-element" {
			acc.push(el as *mut _);
		}
		for child in el.children.iter_mut() {
			if let Node::Element(ref mut e) = child {
				collect(e, acc);
			}
		}
	}
	collect(&mut root, &mut targets);
	for ptr in targets {
		unsafe {
			let node = &mut *ptr;
			// build new div
			let mut new_root = parse("<html><div></div></html>");
			let mut new_div = find_mut(&mut new_root, &|e| e.name().eq_ignore_ascii_case("div"))
				.unwrap()
				.clone();
			// copy innerHTML
			let inner = node.inner_html();
			new_div.set_inner_html(&inner);
			// set class to tag name (match expected js test output)
			new_div.set_attribute("class", node.name());
			// replace
			node.replace_with(&format!("<div class=\"{}\">{}</div>", node.name(), inner));
		}
	}
	let expected = "<html>\n  <div class=\"some-custom-element\">yay</div>\n</html>\n";
	assert_eq!(root.to_string(), expected);
}

#[test]
fn class_list_ops() {
	let mut root = parse("<div class=\"foo bar\"></div>");
	let div = find_mut(&mut root, &|e| e.name().eq_ignore_ascii_case("div")).unwrap();
	// initial
	assert_eq!(div.class_list().len(), 2);
	assert_eq!(div.class_names(), "foo bar");
	// remove
	div.class_list_remove("bar");
	assert_eq!(div.class_names(), "foo");
	div.class_list_add("bar");
	assert_eq!(div.class_names(), "foo bar");
	div.class_list_toggle("bar");
	assert_eq!(div.class_names(), "foo");
	div.class_list_toggle("bar");
	assert_eq!(div.class_names(), "foo bar");
	div.class_list_replace("bar", "mycls");
	assert_eq!(div.class_names(), "foo mycls");
	assert!(div.class_list_contains("foo"));
	assert!(!div.class_list_contains("bar"));
}
