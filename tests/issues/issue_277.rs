use node_html_parser::parse;

#[test]
fn issue_277_custom_tag_name() {
	let html = "<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <title>test</title>\n  </head>\n  <body>\n  <template>\n    <h@1>Smile!</h@1>\n  </template>\n  </body>\n</html>";
	let root = parse(html);
	let t = root.query_selector("template").unwrap();
	let el = t.children[1].as_element().unwrap();
	assert_eq!(el.outer_html(), "<h@1>Smile!</h@1>");
}

#[test]
fn issue_277_unicode_tag_name() {
	let html = "<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <title>test</title>\n  </head>\n  <body>\n  <template>\n    <h测试اختبار>Smile!</h测试اختبار>\n  </template>\n  </body>\n</html>";
	let root = parse(html);
	let t = root.query_selector("template").unwrap();
	let el = t.children[1].as_element().unwrap();
	assert_eq!(el.outer_html(), "<h测试اختبار>Smile!</h测试اختبار>");
}
