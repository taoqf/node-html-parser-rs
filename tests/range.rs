use node_html_parser::{parse_with_options, Options};

fn get_first<'a>(
	root: &'a node_html_parser::HTMLElement,
	sel: &str,
) -> &'a node_html_parser::HTMLElement {
	root.query_selector(sel).expect("selector match")
}

#[test]
fn range_basic() {
	let html = "<div><p id='a'>Hi</p><br/><script>if(a<b){}</script></div>";
	let root = parse_with_options(html, &Options::default());
	let div = root.first_element_child().unwrap();
	// p range should cover <p id='a'>Hi</p>
	let p = get_first(div, "#a");
	let pr = p.range().expect("p range");
	let p_slice = &html[pr.0..pr.1];
	assert!(p_slice.starts_with("<p"), "p slice start");
	assert!(p_slice.ends_with("</p>"), "p slice end");
	// text inside p
	let text_node = match p.first_child().unwrap() {
		node_html_parser::Node::Text(t) => t,
		_ => panic!(),
	};
	let tr = text_node.range().expect("text range");
	assert_eq!(&html[tr.0..tr.1], "Hi");
	// br self closing
	let br = p.next_element_sibling().unwrap();
	assert_eq!(br.name(), "br");
	let br_range = br.range().unwrap();
	let br_slice = &html[br_range.0..br_range.1];
	assert!(br_slice.starts_with("<br"), "br slice start");
	// script block
	let script = br.next_element_sibling().unwrap();
	assert_eq!(script.name(), "script");
	let sr = script.range().unwrap();
	let s_slice = &html[sr.0..sr.1];
	assert!(s_slice.starts_with("<script>"));
	assert!(s_slice.ends_with("</script>"));
	// inner text node range
	let inner_txt = match script.first_child().unwrap() {
		node_html_parser::Node::Text(t) => t,
		_ => panic!(),
	};
	let ir = inner_txt.range().unwrap();
	assert_eq!(&html[ir.0..ir.1], "if(a<b){}");
}

#[test]
fn range_unclosed_autofix() {
	let html = "<div><p>abc"; // missing closures
	let root = parse_with_options(html, &Options::default());
	let div = root.first_element_child().unwrap();
	let p = div.first_element_child().unwrap();
	let r = p.range().unwrap();
	// should extend to end of source because auto-fix closes it
	assert_eq!(r.1, html.len());
}
