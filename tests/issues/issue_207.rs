use node_html_parser::{parse_with_options, Options};

#[test]
fn issue_207_void_tag_custom_and_closing_slash() {
	// 配置自定义 void 标签列表（覆盖默认列表，因此需要把默认也一起列出）
	let html = "<custom-void></custom-void><img src=\"a.png\">";
	let mut opts = Options::default();
	opts.void_tag.add_closing_slash = true;
	opts.void_tag.tags = Some(
		vec![
			// 默认集
			"area",
			"base",
			"br",
			"col",
			"embed",
			"hr",
			"img",
			"input",
			"link",
			"meta",
			"param",
			"source",
			"track",
			"wbr",
			// 自定义
			"custom-void",
		]
		.into_iter()
		.map(|s| s.to_string())
		.collect(),
	);
	let root = parse_with_options(html, &opts);
	// 期望：custom-void 被视为 void，自闭合且添加斜杠；img 仍为 void
	// 当前序列化 custom-void 自闭合格式无空格 (<custom-void/>); 与 JS 可能存在微差异（JS 输出含空格）。
	assert_eq!(root.to_string(), "<custom-void/><img src=\"a.png\" />");
}
