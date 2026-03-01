use node_html_parser::{parse_with_options, Options};

#[test]
fn script_block_preserves_raw_inner() {
	let opts = Options::default();
	// script 默认在 block_text_elements 中，值为 true 表示忽略内部解析
	let html = "<div><script>if (a < b && c > d) { console.log('</p> not tag'); }</script><p id='x'>T</p></div>";
	let root = parse_with_options(html, &opts);
	let div = root.first_element_child().unwrap();
	let script = div.first_element_child().unwrap();
	assert_eq!(script.name(), "script");
	let raw = script.inner_html();
	assert!(
		raw.contains("if (a < b && c > d) { console.log('</p> not tag'); }"),
		"script inner lost: {raw}"
	);
	// 确认后续 p 未被吞掉
	let ps: Vec<&node_html_parser::HTMLElement> = div
		.children_elements()
		.into_iter()
		.filter(|e| e.name() == "p")
		.collect();
	assert_eq!(ps.len(), 1, "p element should remain after script block");
}

#[test]
fn style_block_entity_and_tag_like_text() {
	let opts = Options::default();
	let html = "<style>.cls > p { content: '<span>'; }</style>";
	let root = parse_with_options(html, &opts);
	let style = root.first_element_child().unwrap();
	let raw = style.inner_html();
	assert!(
		raw.contains(".cls > p { content: '<span>'; }"),
		"style content mismatch: {raw}"
	);
	assert!(
		style.children_elements().is_empty(),
		"style should not parse nested span"
	);
}
