use node_html_parser::{parse_with_options, Node, Options};

#[test]
fn issue_203_code_not_none_in_block_text_elements() {
	// In JS test, code element inside pre should be preserved and accessible
	let html = "<pre><code class=\"language-js\">const a = 1;</code></pre>";
	// 禁用 pre 的 raw 模式以便解析内部 <code> 标签（JS 对应测试配置 blockTextElements 不包含 pre）
	let mut opts = Options::default();
	opts.block_text_elements.remove("pre");
	let mut root = parse_with_options(html, &opts);
	// 手动遍历找到 code 元素的可变引用
	fn find_code<'a>(
		el: &'a mut node_html_parser::HTMLElement,
	) -> Option<&'a mut node_html_parser::HTMLElement> {
		for child in el.children.iter_mut() {
			if let Node::Element(e) = child {
				if e.name() == "code" {
					return Some(e);
				}
				if let Some(found) = find_code(e) {
					return Some(found);
				}
			}
		}
		None
	}
	let pre = root.first_element_child_mut().unwrap();
	let code_el = find_code(pre).expect("code element present");
	assert_eq!(code_el.get_attribute("class"), Some("language-js".into()));
}
