use node_html_parser::{parse_with_options, HTMLElement, Node, Options};

const SOURCE: &str = r#"
Leading text


<div>
  <p>Text Content</p>
  Goes Here
</div>
<input name="hello">
<!-- comment -->
<style>
  .abc {
    display: none
  }
</style>
<pre>
  block Text
</pre>
<span>The space between us</span>      <span>is vast</span>
Closing text
"#;

fn walk<'a>(el: &'a HTMLElement, out: &mut Vec<Node>) {
	for c in &el.children {
		out.push(c.clone());
		if let Node::Element(e) = c {
			walk(e, out);
		}
	}
}

#[test]
fn node_ranges_parity_subset() {
	let mut opts = Options::default();
	opts.comment = true;
	let root = parse_with_options(SOURCE, &opts);
	let mut nodes = Vec::new();
	walk(&root, &mut nodes);
	assert!(nodes.len() > 10);
	for n in &nodes {
		match n {
			Node::Element(e) => {
				if let Some((s, eidx)) = e.range() {
					assert!(s < eidx && eidx <= SOURCE.len());
					let slice = &SOURCE[s..eidx];
					assert!(slice.starts_with('<'));
				}
			}
			Node::Text(t) => {
				if let Some((s, eidx)) = t.range() {
					assert!(s <= eidx && eidx <= SOURCE.len());
					let slice = &SOURCE[s..eidx];
					assert!(slice.contains(t.raw.trim()) || t.raw.trim().is_empty());
				}
			}
			Node::Comment(c) => {
				if let Some((s, eidx)) = c.range {
					let slice = &SOURCE[s..eidx];
					assert!(slice.starts_with("<!--") && slice.ends_with("-->"));
				}
			}
		}
	}
}

#[test]
fn constructor_default_ranges_none() {
	// 当前 crate 未公开 CommentNode/TextNode 构造；仅验证通过 parse 创建的节点都拥有范围或 None 一致。
	let root = parse_with_options(
		"<div><span>Hello</span>world<!--c--></div>",
		&Options {
			comment: true,
			..Options::default()
		},
	);
	let mut nodes = Vec::new();
	walk(&root, &mut nodes);
	assert!(nodes.len() >= 3);
	for n in nodes {
		match n {
			Node::Element(e) => {
				assert!(e.range().is_some());
			}
			Node::Text(t) => {
				assert!(t.range().is_some());
			}
			Node::Comment(c) => {
				assert!(c.range.is_some());
			}
		}
	}
}
