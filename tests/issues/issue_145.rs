use node_html_parser::{parse_with_options, Options};

// issue 145: Angular 风格事件与模板引用属性解析
#[test]
fn issue_145_angular_like_attributes() {
	let html = "<input matInput (keyup)=\"applyFilter($event)\" placeholder=\"Ex. IMEI\" #input>";
	let root = parse_with_options(html, &Options::default());
	let input = root.first_element_child().expect("input element");
	let mut cloned = input.clone();
	assert_eq!(cloned.get_attribute("#input").as_deref(), Some(""));
	assert_eq!(
		cloned.get_attribute("(keyup)").as_deref(),
		Some("applyFilter($event)")
	);
	assert_eq!(
		cloned.get_attribute("placeholder").as_deref(),
		Some("Ex. IMEI")
	);
	// 序列化：我们当前会输出 <input ...>（void tag 无自闭合斜杠）
	// 接受可能的两种形式以保持与现有 void 行为兼容。
	let s = input.to_string();
	assert!(
		s == html || s == format!("{}", html.replace(">", "/>")),
		"unexpected serialization: {}",
		s
	);
}
