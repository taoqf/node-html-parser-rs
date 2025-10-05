use node_html_parser::{parse_with_options, Node, Options};

// 辅助函数：递归删除第一个匹配标签
fn remove_first_tag(root: &mut node_html_parser::HTMLElement, tag: &str) -> bool {
	for child in root.children.iter_mut() {
		if let Node::Element(e) = child {
			if e.name() == tag {
				e.remove();
				return true;
			}
			if remove_first_tag(e, tag) {
				return true;
			}
		}
	}
	false
}

#[test]
fn issue_186_inner_html_replace_children() {
	let mut root = parse_with_options(
		"<ul id=\"list\"><li>Hello World</li></ul>",
		&Options::default(),
	);
	{
		let ul = root.first_element_child_mut().unwrap();
		let li = ul.first_element_child_mut().unwrap();
		li.set_inner_html("<a href=\"#\">Some link</a>");
	}
	assert_eq!(
		root.to_string(),
		"<ul id=\"list\"><li><a href=\"#\">Some link</a></li></ul>"
	);
	{
		let ul = root.first_element_child_mut().unwrap();
		remove_first_tag(ul, "a");
	}
	assert_eq!(root.to_string(), "<ul id=\"list\"><li></li></ul>");
}

#[test]
fn issue_186_set_content_replace_children() {
	let mut root = parse_with_options(
		"<ul id=\"list\"><li>Hello World</li></ul>",
		&Options::default(),
	);
	{
		let ul = root.first_element_child_mut().unwrap();
		let li = ul.first_element_child_mut().unwrap();
		li.set_content("<a href=\"#\">Some link</a>");
	}
	assert_eq!(
		root.to_string(),
		"<ul id=\"list\"><li><a href=\"#\">Some link</a></li></ul>"
	);
	{
		let ul = root.first_element_child_mut().unwrap();
		remove_first_tag(ul, "a");
	}
	assert_eq!(root.to_string(), "<ul id=\"list\"><li></li></ul>");
}

#[test]
fn issue_186_replace_with_text() {
	let mut root = parse_with_options(
		"<ul id=\"list\"><li><a href=\"#\">Some link</a></li></ul>",
		&Options::default(),
	);
	{
		let ul = root.first_element_child_mut().unwrap();
		let li = ul.first_element_child_mut().unwrap();
		// find <a>
		let a_index = li
			.children
			.iter()
			.position(|n| matches!(n, Node::Element(e) if e.name()=="a" ))
			.unwrap();
		if let Node::Element(a_el) = &mut li.children[a_index] {
			a_el.replace_with("Hello World");
		}
	}
	assert_eq!(
		root.to_string(),
		"<ul id=\"list\"><li>Hello World</li></ul>"
	);
}

#[test]
fn issue_186_insert_adjacent_html_afterbegin() {
	let mut root = parse_with_options(
		"<ul id=\"list\"><li><a href=\"#\">Some link</a></li></ul>",
		&Options::default(),
	);
	{
		let ul = root.first_element_child_mut().unwrap();
		let li = ul.first_element_child_mut().unwrap();
		li.insert_adjacent_html("afterbegin", "<b>Hello World</b>")
			.unwrap();
	}
	assert_eq!(
		root.to_string(),
		"<ul id=\"list\"><li><b>Hello World</b><a href=\"#\">Some link</a></li></ul>"
	);
}
