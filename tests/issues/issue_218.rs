use node_html_parser::parse;

// issue 218: attribute value contains quotes & mutation ordering / escaping
#[test]
fn issue_218_attribute_quote_and_updates() {
	let html = "<html>\n  <div id=\"_\" title='\"world\"' onClick='alert(\"hello\")' color=\"red\">nochange</div>\n  <div id=\"e\" title='\"world\"' color=\"red\">expected</div>\n  <div id=\"a\" title='\"world\"' onClick='alert(\"hello\")' color=\"red\">actual</div>\n</html>";
	let mut root = parse(html);
	// 在真实 DOM 上原地修改（与 JS 版行为对齐）
	{
		let e = root.get_element_by_id_mut("e").expect("#e exists");
		e.set_attribute("onClick", "alert('hello')");
	}
	{
		let a = root.get_element_by_id_mut("a").expect("#a exists");
		a.remove_attribute("color");
		a.set_attribute("title", "\"replaced\"");
	}
	// 由于 set_attribute/ remove_attribute 会统一使用双引号并转义内部双引号为 &quot;
	// 根据当前实现：#e 新增 onClick 排在最后；#a 删除 color 后 title 被替换，保留 onClick。
	let serialized = root.to_string();
	println!("serialized=\n{}", serialized);
	// e: 所有属性被标准化为双引号，内部双引号转义为 &quot;
	assert!(serialized.contains("<div id=\"e\" title=\"&quot;world&quot;\" color=\"red\" onClick=\"alert('hello')\">expected</div>"));
	// a: color 被移除，title 被替换并转义，onClick 内部双引号被转义；属性顺序可能变化，单独检查 outer_html 组成
	let a_el = root.query_selector("#a").unwrap();
	let a_html = a_el.outer_html();
	assert!(a_html.starts_with("<div "));
	assert!(a_html.contains("id=\"a\""));
	assert!(a_html.contains("title=\"&quot;replaced&quot;\""));
	assert!(a_html.contains("onClick=\"alert(&quot;hello&quot;)\""));
	assert!(a_html.ends_with(">actual</div>"));
}

#[test]
fn issue_218_escape_newlines() {
	let root = parse("<p></p>");
	let mut p = root.query_selector("p").unwrap().clone();
	p.set_attribute("a", "1\n2");
	assert_eq!(p.get_attr("a").unwrap(), "1\n2");
	assert_eq!(p.outer_html(), "<p a=\"1\n2\"></p>");
}
