use node_html_parser::parse;

#[test]
fn issue_51_attribute_value_with_gt() {
	let html = r#"<template v-if="list.length>0"> <div>123</div> </template>"#;
	let root = parse(html);
	assert_eq!(root.outer_html(), html);
	let tpl = root.first_child().and_then(|n| n.as_element()).unwrap();
	assert_eq!(tpl.get_attr("v-if"), Some("list.length>0"));
	// raw_attrs_str 保留原始引号与大小写
	assert_eq!(tpl.raw_attrs_str(), "v-if=\"list.length>0\"");
	assert_eq!(tpl.name().to_ascii_uppercase(), "TEMPLATE");
	let div = tpl.children.iter().find_map(|c| c.as_element());
	assert_eq!(div.unwrap().text(), "123");
}
