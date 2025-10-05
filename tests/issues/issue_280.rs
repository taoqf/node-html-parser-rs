use node_html_parser::parse;
use std::fs;

// JS 原始测试 skip（性能回归比较多个版本）。这里添加一个精简检测：
// 1. 读取 HTML Standard 大文件（如果存在）
// 2. 解析后找到 <title> 节点文本包含 "HTML Standard".
// 若资产缺失则跳过（通过 `return`).

#[test]
fn issue_280_large_html_standard_title() {
	let path = "tests/assets/html/HTML Standard.html";
	let html = match fs::read_to_string(path) {
		Ok(c) => c,
		Err(_) => return, // asset 不在仓库时忽略
	};
	let doc = parse(&html);
	if let Some(title) = doc.query_selector("title") {
		assert!(
			title.text().contains("HTML Standard"),
			"title 应包含 'HTML Standard'"
		);
	} else {
		panic!("未找到 <title> 节点");
	}
}
