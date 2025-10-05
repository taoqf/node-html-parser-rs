use node_html_parser::{parse_with_options, Options};

// issue 152: parseNoneClosedTags 行为
// JS 源测试第一个用例被 skip；此处仅实现第二个（开启 parseNoneClosedTags）
#[test]
fn issue_152_parse_none_closed_tags() {
	let html = "<div>\n<div id=\"chr-content\">\n<span>\n  lkjasdkjasdkljakldj\n</div>\n</div>";
	let mut opts = Options::default();
	opts.parse_none_closed_tags = true;
	let root = parse_with_options(html, &opts);
	// JS 期望：在 parseNoneClosedTags=true 下，修复为补齐 </span> 并调整嵌套。
	let expected =
		"<div>\n<div id=\"chr-content\">\n<span>\n  lkjasdkjasdkljakldj\n</span></div>\n</div>";
	assert_eq!(root.to_string(), expected);
}
