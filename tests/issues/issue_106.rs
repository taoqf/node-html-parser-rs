use node_html_parser::parse;

// issue 106: memory leak stress test (JS version skipped). We keep a light ignored parity test.
#[test]
#[ignore]
fn issue_106_memory_leak_smoke() {
	let content = std::fs::read_to_string(
		"tests/assets/html/view-source_https___epicentrk.ua_shop_kirpich-ogneupornyy_.html",
	)
	.expect("asset html present");
	let root = parse(&content);
	// Try the two selector paths from JS
	let title = root
		.query_selector(".shop-categories__title")
		.or_else(|| root.query_selector(".headList h1"));
	if let Some(t) = title {
		let text = t.raw_text();
		assert!(
			text.contains("Кирпич"),
			"expected Cyrillic word in title, got: {}",
			text
		);
	}
}
