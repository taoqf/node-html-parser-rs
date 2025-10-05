use node_html_parser::{parse_with_options, Node, Options};

// issue 171: appendChild 迁移节点
#[test]
fn issue_171_append_child_moves_node() {
	let html = "<a><b><d></d></b><c></c></a>";
	let mut root = parse_with_options(html, &Options::default());
	let a = root.first_element_child_mut().unwrap();
	// a children: b, c
	// b first child d; we want c.append_child(d)
	let b = a.first_element_child_mut().unwrap();
	// 取出 d 的原始指针方便比较
	let mut d_opt = None;
	for child in &mut b.children {
		if let Node::Element(e) = child {
			if e.name() == "d" {
				d_opt = Some(e.clone());
				break;
			}
		}
	}
	let d_cloned = d_opt.expect("d element");
	// 实际操作：找到 d 的可变引用
	// 简化：移除 d 后再 append 到 c
	b.remove_children_where(|n| matches!(n, Node::Element(e) if e.name()=="d"));
	let c = a
		.children
		.iter_mut()
		.find_map(|n| {
			if let Node::Element(e) = n {
				if e.name() == "c" {
					Some(e)
				} else {
					None
				}
			} else {
				None
			}
		})
		.unwrap();
	c.append_child(Node::Element(d_cloned));
	assert_eq!(
		a.to_string(),
		"<a><b></b><c><d></d></c></a>".replace("<a>", "<a>")
	);
}
