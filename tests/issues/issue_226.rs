use node_html_parser::{parse, Node};

#[test]
fn issue_226_get_node_line_numbers() {
	let html = "<div>\n\t<img src=\"http://localhost/foo.png\" />\n\t<img src=\"http://localhost/bar.png\" />\n\t<img src=\"http://localhost/foo.png\" />\n\t<img src=\"./foo.png\" ></img>\n\n\t<img src=\"./bar.png\" ></img>\n\t<img src=\"./foo.png\" ></img>\n</div>";
	let mut root = parse(html);
	// 需要可变 get_attribute；手动 DFS 获取 img 可变引用
	fn collect_imgs<'a>(
		el: &'a mut node_html_parser::HTMLElement,
		out: &mut Vec<*mut node_html_parser::HTMLElement>,
	) {
		// 手动索引避免同时持有多个 &mut 引用造成借用冲突，先收集裸指针。
		let len = el.children.len();
		for i in 0..len {
			let ptr: *mut Node = &mut el.children[i];
			unsafe {
				if let Node::Element(e) = &mut *ptr {
					if e.name() == "img" {
						out.push(&mut **e as *mut _);
					}
				}
				if let Node::Element(e) = &mut *ptr {
					collect_imgs(e, out);
				}
			}
		}
	}
	let mut img_ptrs: Vec<*mut node_html_parser::HTMLElement> = Vec::new();
	if let Some(root_el) = root.first_element_child_mut() {
		collect_imgs(root_el, &mut img_ptrs);
	}
	fn get_line(html: &str, start: usize) -> usize {
		html[..start].chars().filter(|c| *c == '\n').count() + 1
	}
	let mut lines: Vec<usize> = Vec::new();
	for ptr in img_ptrs {
		let img = unsafe { &mut *ptr };
		let src = img.get_attribute("src").unwrap_or_default();
		let (start, _) = img.range().expect("range");
		let line = get_line(html, start);
		if src.starts_with("http://localhost") || !src.starts_with("http") {
			lines.push(line);
		}
	}
	assert_eq!(lines, vec![2, 3, 4, 5, 7, 8]);
}
