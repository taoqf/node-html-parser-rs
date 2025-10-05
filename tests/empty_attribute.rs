use node_html_parser::{parse_with_options, Options};

fn first_div(html: &str, opts: &Options) -> Box<node_html_parser::HTMLElement> {
	parse_with_options(html, opts)
		.first_element_child()
		.unwrap()
		.clone_box()
}

trait CloneBoxEl {
	fn clone_box(&self) -> Box<node_html_parser::HTMLElement>;
}
impl CloneBoxEl for node_html_parser::HTMLElement {
	fn clone_box(&self) -> Box<node_html_parser::HTMLElement> {
		Box::new(self.clone())
	}
}

#[test]
fn attr_without_value() {
	let div = first_div("<div foo></div>", &Options::default());
	// get_attribute 需要可变借用；复制一份独立测试
	let mut d = (*div).clone();
	assert_eq!(d.get_attribute("foo").as_deref(), Some(""));
	assert_eq!(div.to_string(), "<div foo></div>");
}

#[test]
fn attr_with_empty_value() {
	let div = first_div("<div foo=\"\"></div>", &Options::default());
	let mut d = (*div).clone();
	assert_eq!(d.get_attribute("foo").as_deref(), Some(""));
	assert_eq!(div.to_string(), "<div foo=\"\"></div>");
}

#[test]
fn empty_class_value() {
	let div = first_div("<div class=\"\"></div>", &Options::default());
	let mut d = (*div).clone();
	assert_eq!(d.get_attribute("class").as_deref(), Some(""));
	// classNames => class_list tokens；当前未直接公开 length API，以 split 验证
	assert!(d.class_names().is_empty());
	assert_eq!(div.to_string(), "<div class=\"\"></div>");
}

#[test]
fn attribute_name_not_exist() {
	let div = first_div("<div class=\"\"></div>", &Options::default());
	let mut d = (*div).clone();
	assert_eq!(d.get_attribute("foo"), None);
	assert!(d.class_names().is_empty());
	assert_eq!(div.to_string(), "<div class=\"\"></div>");
}
