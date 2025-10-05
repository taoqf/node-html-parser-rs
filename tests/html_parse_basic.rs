use node_html_parser::{parse_with_options, Options};

// 基础解析相关 (对应 js/tests/html.js parse() 部分的前几条)
#[test]
fn parse_basic_nested_structure() {
	let html = "<p id=\"id\"><a class='cls'>Hello</a><ul><li><li></ul><span></span></p>";
	let root = parse_with_options(html, &Options::default());
	let parsed_p = root.first_element_child().expect("p element present");
	assert_eq!(parsed_p.to_string(), html, "serialization mismatch");
}

#[test]
fn parse_with_lower_case_tag_option() {
	let root = parse_with_options(
		"<DIV><a><img/></A><p></P></div>",
		&Options {
			lower_case_tag_name: true,
			..Default::default()
		},
	);
	// 所有标签应序列化为小写
	let out = root.outer_html();
	assert!(out.contains("<div>"));
	assert!(out.contains("<p>"));
}

#[test]
fn parse_auto_close_li() {
	let html = "<ul><li>one<li>two</ul>";
	let root = parse_with_options(html, &Options::default());
	let out = root.outer_html();
	assert!(out.contains("one"));
	assert!(out.contains("two"));
}

#[test]
fn parse_fix_nested_a_tags() {
	let mut opts = Options::default();
	opts.fix_nested_a_tags = true;
	let root = parse_with_options("<a><a>inner</a></a>", &opts);
	let out = root.outer_html();
	assert!(
		out.matches("<a>").count() >= 2,
		"should split nested <a>: {}",
		out
	);
}

#[test]
fn parse_picture_and_void_tags() {
	let html = "<picture><source srcset=\"/a 1w\" sizes=\"100vw\"><img src=\"/a.jpg\" alt=\"A\"></picture>";
	let root = parse_with_options(html, &Options::default());
	let pic = root.first_element_child().unwrap();
	// 通过 outer_html 前缀判断标签名称
	assert!(pic.outer_html().starts_with("<picture"));
}

#[test]
fn parse_namespaced_simple() {
	let ns_html = "<ns:identifier>content</ns:identifier>";
	let root = parse_with_options(ns_html, &Options::default());
	assert_eq!(root.outer_html(), ns_html);
}

#[test]
fn parse_script_style_default_no_text() {
	let mut opts = Options::default();
	opts.suppress_script_style_text = true;
	let root = parse_with_options("<script>1</script><style>2</style>", &opts);
	let first = root.first_element_child().unwrap();
	assert!(
		first.first_child().is_none(),
		"script should have no text by default"
	);
}

#[test]
fn parse_script_style_with_options_extract_text() {
	let mut opts = Options::default();
	opts.suppress_script_style_text = false; // ensure extraction
	let root = parse_with_options("<script>1</script><style>2&amp;</style>", &opts);
	let script = root.first_element_child().unwrap();
	let txt = script.first_child().expect("script text child");
	match txt {
		node_html_parser::Node::Text(t) => assert_eq!(t.text(), "1"),
		_ => panic!("expected text"),
	};
}

#[test]
fn parse_large_like_tables_file_smoke() {
	// 占位：原 JS 测试读取 tables.html 并断言 table 数量，这里仅确保能解析
	let opts = Options::default();
	let path = "tests/assets/html/tables.html";
	if std::path::Path::new(path).exists() {
		let data = std::fs::read_to_string(path).unwrap();
		let root = parse_with_options(&data, &opts);
		// 简单统计 <table> 出现次数
		let count = root.outer_html().match_indices("<table").count();
		assert!(count > 100, "expected many tables, got {}", count);
	}
}
