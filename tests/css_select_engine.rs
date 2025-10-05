use node_html_parser::{
	compile_experimental, parse_with_options, CssHtmlAdapter, CssSelectOptions, Options,
};

fn collect_ids<'a>(els: Vec<&'a node_html_parser::HTMLElement>) -> Vec<String> {
	els.into_iter()
		.map(|e| e.get_attr("id").unwrap_or("").to_string())
		.collect()
}

#[test]
fn experimental_basic_equivalence() {
	let html = "<div id='r'><section id='s1'><p id='p1' class='a b'><span id='sp1'></span></p><p id='p2' class='b'><em id='em1'></em></p></section><section id='s2'><p id='p3' class='a'><span id='sp2'></span></p></section></div>";
	let root = parse_with_options(html, &Options::default());
	let adapter = CssHtmlAdapter::new(&root);
	let sel_opts = CssSelectOptions::<CssHtmlAdapter>::default();
	// 测试一组代表性选择器
	let cases = vec![
		("p.a", vec!["p1", "p3"]),
		("section > p.b", vec!["p1", "p2"]),
		("section p > span", vec!["sp1", "sp2"]),
		("p.a:not(.b)", vec!["p3"]),
		("p:is(.a,.b)", vec!["p1", "p2", "p3"]),
		("p:where(.a)", vec!["p1", "p3"]),
		("p:has(span)#p1", vec!["p1"]),
		(":scope > section > p:first-child", vec!["p1", "p3"]),
		("p:nth-child(2)", vec!["p2"]),
		("p:nth-of-type(2)", vec!["p2"]),
	];
	for (expr, expected_ids) in cases {
		let q = compile_experimental::<CssHtmlAdapter>(expr, &sel_opts, &adapter);
		// 遍历所有节点取匹配
		let mut found = Vec::new();
		fn walk<'a>(
			cur: &'a node_html_parser::HTMLElement,
			acc: &mut Vec<&'a node_html_parser::HTMLElement>,
		) {
			if !cur.is_root() {
				acc.push(cur);
			}
			for c in cur.iter_elements() {
				walk(c, acc);
			}
		}
		let mut all = Vec::new();
		walk(&root, &mut all);
		for el in all {
			if (q.func)(el) {
				found.push(el);
			}
		}
		let got = collect_ids(found);
		assert_eq!(got, expected_ids, "selector {} mismatch", expr);
	}
}
