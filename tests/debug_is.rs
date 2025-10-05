use node_html_parser::{
	compile_experimental, parse_with_options, CssHtmlAdapter, CssSelectOptions, Options,
};

#[test]
fn debug_is_branch() {
	let html = "<div id='r'><section id='s1'><p id='p1' class='a b'><span id='sp1'></span></p><p id='p2' class='b'><em id='em1'></em></p></section><section id='s2'><p id='p3' class='a'><span id='sp2'></span></p></section></div>";
	let root = parse_with_options(html, &Options::default());
	let adapter = CssHtmlAdapter::new(&root);
	let sel_opts = CssSelectOptions::<CssHtmlAdapter>::default();
	let q_pb = compile_experimental::<CssHtmlAdapter>("p.b", &sel_opts, &adapter);
	let q_is = compile_experimental::<CssHtmlAdapter>("p:is(.a,.b)", &sel_opts, &adapter);

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
	let mut pb_ids = Vec::new();
	let mut is_ids = Vec::new();
	for el in &all {
		if (q_pb.func)(el) {
			pb_ids.push(el.get_attr("id").unwrap_or(""));
		}
	}
	for el in &all {
		if (q_is.func)(el) {
			is_ids.push(el.get_attr("id").unwrap_or(""));
		}
	}
	assert_eq!(pb_ids, vec!["p1", "p2"], "p.b mismatch");
	assert_eq!(is_ids, vec!["p1", "p2", "p3"], "p:is(.a,.b) mismatch");
}
