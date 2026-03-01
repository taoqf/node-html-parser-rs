use node_html_parser::parse;
use std::fs;
use std::time::Instant;

// JS 原始测试 skip（性能回归比较多个版本）。这里添加一个精简检测：
// 1. 读取 HTML Standard 大文件（如果存在）
// 2. 解析后找到 <title> 节点文本包含 "HTML Standard".
// 若资产缺失则跳过（通过 `return`).

#[test]
fn issue_280_large_html_standard_title() {
	let path = "tests/assets/html/HTML Standard.html";

	// 测量文件读取时间
	let read_start = Instant::now();
	let html = match fs::read_to_string(path) {
		Ok(c) => c,
		Err(_) => return, // asset 不在仓库时忽略
	};
	let read_duration = read_start.elapsed();
	println!(
		"📁 文件读取时间: {:.3}s (文件大小: {:.2}MB)",
		read_duration.as_secs_f64(),
		html.len() as f64 / 1_000_000.0
	);

	// 测量HTML解析时间
	let parse_start = Instant::now();
	let doc = parse(&html);
	let parse_duration = parse_start.elapsed();
	println!("🔄 HTML解析时间: {:.3}s", parse_duration.as_secs_f64());

	// 测量查询时间
	let query_start = Instant::now();
	if let Some(title) = doc.query_selector("title") {
		let query_duration = query_start.elapsed();
		println!("🔍 标题查询时间: {:.3}s", query_duration.as_secs_f64());

		assert!(
			title.text().contains("HTML Standard"),
			"title 应包含 'HTML Standard'"
		);
		println!("✅ 找到标题: {}", title.text().trim());
	} else {
		panic!("未找到 <title> 节点");
	}

	println!("📊 总时间分布:");
	println!(
		"  - 读取文件: {:.1}%",
		read_duration.as_secs_f64() / (read_duration + parse_duration).as_secs_f64() * 100.0
	);
	println!(
		"  - 解析HTML: {:.1}%",
		parse_duration.as_secs_f64() / (read_duration + parse_duration).as_secs_f64() * 100.0
	);
}
