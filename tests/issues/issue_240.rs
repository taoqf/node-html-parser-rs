use node_html_parser::parse;

#[test]
fn issue_240_multiline_attribute_preserved_and_append() {
	let html = "<div unchanged='[\npreserve newline\n]'></div>";
	let mut root = parse(html);
	let div = root.first_element_child_mut().unwrap();
	assert_eq!(div.to_string(), html);
	div.set_attribute("append", "newAttribute");
	let out = div.to_string();
	// 原值中单引号属性被重建为双引号且换行保留（当前实现 quote_attribute 可能转为双引号 + &quot; 处理, 若行为不同可调整断言）
	assert!(out.contains("unchanged=\"[\npreserve newline\n]\""));
	assert!(out.contains("append=\"newAttribute\""));
}
