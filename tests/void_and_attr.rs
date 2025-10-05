use node_html_parser::{parse_with_options, Options};

#[test]
fn void_tag_closing_slash_serialization() {
	let mut opts = Options::default();
	opts.void_tag.add_closing_slash = true; // 需要确保该字段可访问，若不可则此测试需调整库导出
	let root = parse_with_options(
		"<br><img src=\"a b\" class=space><div id=\"x\" data=\"v\"></div>",
		&opts,
	);
	let html = root.inner_html();
	assert!(html.contains("<br/>"), "expected <br/> got {html}");
	assert!(
		html.contains("<img src=\"a b\" class=\"space\" />"),
		"img serialization mismatch: {html}"
	);
}

#[test]
fn attribute_quoting_like_js() {
	let opts = Options::default();
	let root = parse_with_options("<div id=\"a\"></div>", &opts);
	let el = root.query_selector("div").unwrap();
	let mut cloned = el.clone();
	// 含换行与制表符
	cloned.set_attribute("data", "line1\nline2\t\"q\"");
	let out = cloned.outer_html();
	// 期望内部包含实际换行与制表符，而非反斜杠转义序列。这里构造期望片段：
	let expected_value = format!("line1\nline2\t{}", "&quot;q&quot;");
	assert!(
		out.contains(&format!("data=\"{}\"", expected_value)),
		"quoting mismatch: {out}"
	);
}
