use node_html_parser::parse;

#[test]
fn issue_249_br_turns_into_newline_in_inner_text() {
	let root = parse("<div>Hello<br>World</div>");
	let div = root.query_selector("div").unwrap();
	// 现在 raw_text() 已对 <br> 映射为 "\n"，应与 JS innerText 行为一致
	let t = div.text();
	assert_eq!(t, "Hello\nWorld", "<br> 应被转换为换行");
}
