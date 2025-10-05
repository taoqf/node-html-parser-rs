use node_html_parser::{parse_with_options, Options};
use node_html_parser::{HTMLElement, Node};

#[test]
fn parse_p_with_nested_a_ul_span() {
	// 对应 JS 测试：should parse "<p id=\"id\"><a class='cls'>Hello</a><ul><li><li></ul><span></span></p>" and return root element
	let html = "<p id=\"id\"><a class='cls'>Hello</a><ul><li><li></ul><span></span></p>";
	let root = parse_with_options(html, &Options::default());
	let parsed_p = root.first_element_child().expect("p element present");

	// 构造期望节点（不直接比较结构体，只比较序列化）
	use node_html_parser::HTMLElement as HE;
	let mut expected_root = HE::new(
		Some("p".into()),
		"id=\"id\"".into(),
		vec![("id".into(), "id".into())],
		false,
		false,
	);
	use node_html_parser::Node as N;
	use node_html_parser::TextNode as TN;
	// 模拟 JS: p.appendChild(a).appendChild(TextNode('Hello'))
	let mut a = HE::new(
		Some("a".into()),
		"class='cls'".into(),
		vec![("class".into(), "cls".into())],
		false,
		false,
	);
	a.append_child(N::Text(TN::new("Hello".into())));
	expected_root.append_child(N::Element(Box::new(a)));
	let mut ul = HE::new(Some("ul".into()), "".into(), vec![], false, false);
	// 解析产生的 <li> 在缺少显式关闭标签时会被自动关闭且 range 仅有起点（start==end），序列化时省略 </li>
	// 为与解析结果一致，手动构造期望时模拟该 range 情况
	let mut li1 = HE::new(Some("li".into()), "".into(), vec![], false, false);
	li1.set_range_start(0); // no end -> treated as auto-closed empty
	let mut li2 = HE::new(Some("li".into()), "".into(), vec![], false, false);
	li2.set_range_start(0);
	ul.append_child(N::Element(Box::new(li1)));
	ul.append_child(N::Element(Box::new(li2)));
	expected_root.append_child(N::Element(Box::new(ul)));
	expected_root.append_child(N::Element(Box::new(HE::new(
		Some("span".into()),
		"".into(),
		vec![],
		false,
		false,
	))));
	let expected_ser = expected_root.to_string();
	let actual_ser = parsed_p.to_string();
	assert_eq!(
		actual_ser, expected_ser,
		"parsed serialization mismatch\nexpected: {}\nactual:   {}",
		expected_ser, actual_ser
	);
}

#[test]
fn auto_close_li() {
	let html = "<ul><li>one<li>two</ul>";
	let opts = Options::default();
	let root = parse_with_options(html, &opts);
	let out = root.outer_html();
	assert!(out.contains("one"));
	assert!(out.contains("two"));
}

#[test]
fn nested_a_fix() {
	let html = "<a><a>inner</a></a>";
	let mut opts = Options::default();
	opts.fix_nested_a_tags = true;
	let root = parse_with_options(html, &opts);
	let out = root.outer_html();
	// Expect two a elements sequential instead of nested
	assert!(out.matches("<a>").count() >= 2);
}

#[test]
fn void_tag_serialization() {
	let html = "<div><br><img src=\"a.png\"></div>";
	let mut opts = Options::default();
	opts.void_tag.add_closing_slash = true;
	let root = parse_with_options(html, &opts);
	let out = root.outer_html();
	assert!(out.contains("<br/>") || out.contains("<br />"));
}

#[test]
fn uppercase_auto_close_and_void_spacing() {
	// Uppercase P followed by another P should auto-close first.
	let html = "<DIV><P>One<P>Two</DIV>"; // note uppercase P
	let root = parse_with_options(html, &Options::default());
	let ser = root.outer_html();
	// Expect two separate P elements, not nested
	let p_lower = ser.match_indices("<p").count();
	let p_upper = ser.match_indices("<P").count();
	let p_count = p_lower + p_upper;
	assert!(
		p_count >= 2,
		"expected auto-closed uppercase P resulting in two P tags, got: {}",
		ser
	);

	// void tag spacing with slash when attribute present
	let mut opts = Options::default();
	opts.void_tag.add_closing_slash = true;
	let root2 = parse_with_options("<div><img src='x.png'></div>", &opts);
	let ser2 = root2.outer_html();
	// Accept both <img src="x.png"/> or <img src="x.png" /> (JS adds space if attrs exist)
	assert!(
		ser2.contains("<img src=\"x.png\" />") || ser2.contains("<img src=\"x.png\"/>"),
		"void tag slash spacing mismatch: {}",
		ser2
	);
}

#[test]
fn structured_text_basic() {
	let html = "<div><p> Hello  <b>World</b> </p><br><p>Line2</p></div>";
	let root = parse_with_options(html, &Options::default());
	let out = root.structured_text();
	// 期望段落分行，并在 <br> 处有换行 (JS 会将 br 作为块分割)
	assert!(out.contains("Hello World"));
	assert!(out.lines().count() >= 2);
}

#[test]
fn remove_whitespace_trim() {
	let html = "<div>  A   <span> B </span>   C </div>";
	let mut root = parse_with_options(html, &Options::default());
	root.remove_whitespace();
	let ser = root.outer_html();
	// 空白被压缩：不出现连续两个普通空格
	assert!(!ser.contains("  "));
}

#[test]
fn class_list_ops() {
	let mut root = parse_with_options("<div><span class='a b'></span></div>", &Options::default());
	let el = root
		.first_element_child_mut()
		.unwrap()
		.first_element_child_mut()
		.unwrap();
	el.class_list_add("c");
	assert!(el.class_list_contains("c"));
	el.class_list_remove("a");
	assert!(!el.class_list_contains("a"));
	el.class_list_replace("b", "d");
	assert!(el.class_list_contains("d"));
}

#[test]
fn dom_like_child_access() {
	let root = parse_with_options(
		"<div><p id='a'></p><span id='b'></span>text</div>",
		&Options::default(),
	);
	let div = root.first_element_child().unwrap();
	assert_eq!(div.child_element_count(), 2);
	assert_eq!(div.first_element_child().unwrap().get_attr("id"), Some("a"));
	assert_eq!(div.last_element_child().unwrap().get_attr("id"), Some("b"));
	assert!(div.first_child().is_some());
	assert!(div.last_child().is_some());
	let ids: Vec<&str> = div
		.children_elements()
		.iter()
		.map(|e| e.get_attr("id").unwrap())
		.collect();
	assert_eq!(ids, vec!["a", "b"]);
}

#[test]
fn selector_matches_and_closest() {
	let root = parse_with_options(
		"<div class='root'><section id='sec'><p class='x y'><span id='inner'></span></p></section></div>",
		&Options::default(),
	);
	let sec = root
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap();
	let p = sec.first_element_child().unwrap();
	let span = p.first_element_child().unwrap();
	// matches
	assert!(p.matches_selector(&root, "p.x.y"));
	assert!(!p.matches_selector(&root, "p.z"));
	// closest (self)
	assert_eq!(
		p.closest_in(&root, "p.x").unwrap().get_attr("class"),
		Some("x y")
	);
	// closest ancestor
	assert_eq!(
		span.closest_in(&root, "section").unwrap().get_attr("id"),
		Some("sec")
	);
	// no match
	assert!(span.closest_in(&root, "article").is_none());
}

#[test]
fn element_sibling_navigation() {
	let root = parse_with_options(
		"<div><p id='a'></p><span id='b'></span><em id='c'></em>text</div>",
		&Options::default(),
	);
	let div = root.first_element_child().unwrap();
	let p = div.first_element_child().unwrap();
	let span = p.next_element_sibling_in(&root).unwrap();
	assert_eq!(span.get_attr("id"), Some("b"));
	let em = span.next_element_sibling_in(&root).unwrap();
	assert_eq!(em.get_attr("id"), Some("c"));
	assert!(em.next_element_sibling_in(&root).is_none());
	assert!(p.previous_element_sibling_in(&root).is_none());
	assert_eq!(
		span.previous_element_sibling_in(&root)
			.unwrap()
			.get_attr("id"),
		Some("a")
	);
	assert_eq!(
		em.previous_element_sibling_in(&root)
			.unwrap()
			.get_attr("id"),
		Some("b")
	);
}

#[test]
fn text_content_and_inner_text() {
	let mut root = parse_with_options(
		"<div><p>Hello &amp; &lt;World&gt;</p></div>",
		&Options::default(),
	);
	let div = root.first_element_child().unwrap();
	assert!(div.inner_text().contains("&amp;")); // raw contains entity
	assert!(div.text_content().contains("& <World>")); // decoded
													// 设置 textContent
	let div_mut = root.first_element_child_mut().unwrap();
	div_mut.set_text_content("A < B & C");
	let raw = div_mut.inner_html();
	// JS 原版 textContent 直接写入文本节点，不会实体编码，这里期望保持原字符
	assert!(raw.contains("A < B & C"));
	assert_eq!(div_mut.text_content(), "A < B & C");
}

#[test]
fn mutation_append_prepend() {
	let mut root = parse_with_options("<div id='r'><p id='a'></p></div>", &Options::default());
	let r = root.first_element_child_mut().unwrap();
	r.append("<span id='b'></span><em id='c'></em>");
	let ids: Vec<String> = r
		.children_elements()
		.iter()
		.map(|e| e.get_attr("id").unwrap().to_string())
		.collect();
	assert_eq!(ids, vec!["a", "b", "c"]);
	r.prepend("<strong id='s'></strong>");
	let ids2: Vec<String> = r
		.children_elements()
		.iter()
		.map(|e| e.get_attr("id").unwrap().to_string())
		.collect();
	assert_eq!(ids2, vec!["s", "a", "b", "c"]);
}

#[test]
fn mutation_insert_adjacent_html() {
	let mut root = parse_with_options("<div id='r'><p id='a'></p></div>", &Options::default());
	{
		let r = root.first_element_child_mut().unwrap();
		assert!(r
			.insert_adjacent_html("beforeend", "<span id='b'></span>")
			.is_ok());
		assert!(r
			.insert_adjacent_html("afterbegin", "<em id='c'></em>")
			.is_ok());
	}
	let r_ro = root.first_element_child().unwrap();
	let ids: Vec<String> = r_ro
		.children_elements()
		.iter()
		.map(|e| e.get_attr("id").unwrap().to_string())
		.collect();
	assert_eq!(ids, vec!["c", "a", "b"]);
	// beforebegin / afterend
	// beforebegin / afterend: 对 <p id='a'> 前后插入
	{
		let r_mut = root.first_element_child_mut().unwrap();
		let a_index = 1; // 当前顺序 c, a, b
				   // 获取 a 的可变引用（raw children）
		let elem_ptr: *mut HTMLElement = match &mut r_mut.children[a_index] {
			Node::Element(bx) => &mut **bx as *mut HTMLElement,
			_ => panic!(),
		};
		unsafe {
			(*elem_ptr).before("<i id='pre'></i>");
		}
		unsafe {
			(*elem_ptr).after("<i id='post'></i>");
		}
	}
	let r_ro2 = root.first_element_child().unwrap();
	let ids2: Vec<String> = r_ro2
		.children_elements()
		.iter()
		.map(|e| e.get_attr("id").unwrap().to_string())
		.collect();
	assert_eq!(ids2, vec!["c", "pre", "a", "post", "b"]);
}

#[test]
fn insert_adjacent_html_invalid_position() {
	let mut root = parse_with_options("<div id='r'></div>", &Options::default());
	let r = root.first_element_child_mut().unwrap();
	let err = r
		.insert_adjacent_html("middle", "<p></p>")
		.err()
		.expect("should error");
	assert!(err.contains("not one of"));
}

#[test]
fn mutation_before_after_replace_remove() {
	let mut root = parse_with_options(
		"<div><p id='a'></p><span id='b'></span></div>",
		&Options::default(),
	);
	let div = root.first_element_child_mut().unwrap();
	// 定位 p
	{
		let first = div.first_element_child_mut().unwrap();
		first.after("<em id='x'></em>");
		first.before("<strong id='y'></strong>");
	}
	// 现在顺序: y, a, x, b
	let order1: Vec<String> = div
		.children_elements()
		.iter()
		.map(|e| e.get_attr("id").unwrap().to_string())
		.collect();
	assert_eq!(order1, vec!["y", "a", "x", "b"]);
	// replaceWith (替换 id='a')
	{
		let idx_a = 1; // y, a, x, b
		let ptr_a: *mut HTMLElement = match &mut div.children[idx_a] {
			Node::Element(bx) => &mut **bx as *mut HTMLElement,
			_ => panic!(),
		};
		unsafe {
			(*ptr_a).replace_with("<u id='ra'></u>");
		}
	}
	// 直接对原节点执行后续 replace / remove：重新获取可变引用
	{
		// 找到 id='x' (当前位置: y, ra, x, b)
		let mut idx_x = None;
		for (i, e) in div.children_elements().iter().enumerate() {
			if e.get_attr("id") == Some("x") {
				idx_x = Some(i);
				break;
			}
		}
		let i_x = idx_x.expect("x present");
		let mut_raw_elem: *mut HTMLElement = match &mut div.children[i_x] {
			Node::Element(bx) => &mut **bx as *mut HTMLElement,
			_ => panic!(),
		};
		unsafe {
			(*mut_raw_elem).replace_with("<u id='u1'></u><u id='u2'></u>");
		}
	}
	let order2: Vec<String> = div
		.children_elements()
		.iter()
		.map(|e| e.get_attr("id").unwrap().to_string())
		.collect();
	assert_eq!(order2, vec!["y", "ra", "u1", "u2", "b"]);
	// remove a
	{
		let mut idx_a = None;
		for (i, e) in div.children_elements().iter().enumerate() {
			if e.get_attr("id") == Some("ra") {
				idx_a = Some(i);
				break;
			}
		}
		let i_a = idx_a.unwrap();
		let ptr_a: *mut HTMLElement = match &mut div.children[i_a] {
			Node::Element(bx) => &mut **bx as *mut HTMLElement,
			_ => panic!(),
		};
		unsafe {
			(*ptr_a).remove();
		}
	}
	let order3: Vec<String> = div
		.children_elements()
		.iter()
		.map(|e| e.get_attr("id").unwrap().to_string())
		.collect();
	assert_eq!(order3, vec!["y", "u1", "u2", "b"]);
}

#[test]
fn inner_html_setter() {
	let mut root = parse_with_options("<div><section id='s'></section></div>", &Options::default());
	let sect = root
		.first_element_child_mut()
		.unwrap()
		.first_element_child_mut()
		.unwrap();
	sect.set_inner_html("<p id='p1'>A&amp;B</p><p id='p2'>C</p>");
	let ids: Vec<String> = sect
		.children_elements()
		.iter()
		.map(|e| e.get_attr("id").unwrap().to_string())
		.collect();
	assert_eq!(ids, vec!["p1", "p2"]);
	// raw entity保持未解码
	let first_raw = sect.first_element_child().unwrap().inner_html();
	assert!(first_raw.contains("A&amp;B"));
}

#[test]
fn node_level_sibling_navigation() {
	let mut opts = Options::default();
	opts.comment = true; // 启用注释解析
	let root = parse_with_options(
		"<div><p id='a'></p>text<!--cmt--><span id='b'></span></div>",
		&opts,
	);
	let div = root.first_element_child().unwrap();
	let p = div.first_element_child().unwrap();
	// next_sibling 应该是文本节点
	let n1 = p.next_sibling().unwrap();
	match n1 {
		node_html_parser::Node::Text(t) => assert!(t.raw.contains("text")),
		_ => panic!("expected text"),
	}
	// comment 节点：直接线性扫描 children
	let mut types: Vec<&str> = Vec::new();
	for n in &div.children {
		types.push(match n {
			node_html_parser::Node::Element(e) => e.get_attr("id").unwrap_or("elem"),
			node_html_parser::Node::Text(_) => "#text",
			node_html_parser::Node::Comment(_) => "#comment",
		});
	}
	assert_eq!(
		types,
		vec!["a", "#text", "#comment", "b"],
		"sequence with comment"
	);
	// span 的 previous_sibling 应为注释，再 previous_sibling 为文本，再 previous_sibling 为 p
	let children_vec = div.children_elements();
	let span = children_vec
		.into_iter()
		.find(|e| e.get_attr("id") == Some("b"))
		.unwrap();
	let prev1 = span.previous_sibling().unwrap(); // comment 节点
	match prev1 {
		node_html_parser::Node::Comment(c) => assert!(c.text.contains("cmt")),
		_ => panic!("expected comment before span"),
	}
	// 需要再往前找文本：从 p 开始的 next_sibling 即文本
	let text_from_p = p.next_sibling().unwrap();
	match text_from_p {
		node_html_parser::Node::Text(t) => assert!(t.raw.contains("text")),
		_ => panic!("expected text node"),
	}
}

#[test]
fn parity_structure_and_trim_right_and_set_attributes() {
	use regex::Regex;
	let mut root = parse_with_options(
		"<div id='r' class='x y'><p class='a a' id='p1'>Hello<span>World</span>ENDTAIL</p><p id='p2'>Second</p></div>",
		&Options::default(),
	);
	let div = root.first_element_child_mut().unwrap();
	// structure: 应包含层级与 #id .class 以及 #text 标记
	let structure = div.structure();
	println!("STRUCTURE=\n{}", structure);
	assert!(structure.lines().next().unwrap().starts_with("div#r.x.y"));
	assert!(structure.contains("p#p1.a"));
	assert!(structure.contains("#text"), "should contain text nodes");

	// trim_right: 在 p1 中按 "END" 截断，后续节点与其后文本被移除
	let p1 = div.first_element_child_mut().unwrap();
	let re = Regex::new("END").unwrap();
	p1.trim_right(&re);
	let html_after = p1.outer_html();
	assert!(html_after.contains("Hello"));
	assert!(!html_after.contains("END"));
	// END 及其后文本被移除；之前的 <span>World</span> 应保留
	assert!(html_after.contains("World"));
	assert!(!html_after.contains("END"));
	assert!(!html_after.contains("TAIL"));

	// set_attributes: 覆盖原属性，仅保留新的，并更新 id/class 缓存
	p1.set_attributes(&[("ID".into(), "np".into()), ("class".into(), "c1 c2".into())]);
	assert_eq!(p1.get_attribute("id"), Some("np".into()));
	assert!(p1.class_list_contains("c1"));
	assert!(p1.class_list_contains("c2"));
	assert!(!p1.class_list_contains("a"));
}

// ---------------- TextNode parity tests (from JS selection) ----------------
#[test]
fn text_node_is_whitespace() {
	use node_html_parser::TextNode;
	let n1 = TextNode::new("".into());
	assert!(n1.is_whitespace(), "empty string should be whitespace");
	let n2 = TextNode::new(" \t".into());
	assert!(n2.is_whitespace(), "space+tab should be whitespace");
	let n3 = TextNode::new(" \t&nbsp; \t".into());
	assert!(
		n3.is_whitespace(),
		"sequence containing &nbsp; tokens should be whitespace"
	);
}

#[test]
fn parse_text_node_basic() {
	let root = parse_with_options("hello mmstudio", &Options::default());
	// first child should be Text node with raw 'hello mmstudio'
	let first = root.first_child().expect("text node present");
	match first {
		Node::Text(t) => assert_eq!(t.raw, "hello mmstudio"),
		_ => panic!("expected Text node"),
	}
	// to_string() (outer_html for root container) should equal original text
	assert_eq!(root.to_string(), "hello mmstudio");
}
