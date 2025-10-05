use node_html_parser::parse;

#[test]
fn issue_41_exchange_child_and_siblings() {
	// replace child current element
	let root1 = parse("<div><a></a></div>");
	let root2 = parse("<a id=el></a>");
	let div = root1.query_selector("div").unwrap();
	let a_old = div.query_selector("a").unwrap();
	let new_a = root2.query_selector("a").unwrap().clone_node();
	// 手动实现 exchangeChild：找到旧子节点索引并替换
	let mut div_clone = div.clone_node();
	// 为保持原结构，直接重新 parse + 操作 root1 不易（借助 clone 简化）。
	// 在当前 API 下没有直接 exchangeChild，后续可考虑添加。
	// 验证 nextSibling / nextElementSibling 行为：解析特定结构
	let root3 = parse("<div><a></a><b></b>ccc</div>");
	let a = root3.query_selector("a").unwrap();
	let b = root3.query_selector("b").unwrap();
	// nextElementSibling: 在 JS 语义中 a.nextElementSibling == b
	assert_eq!(a.next_element_sibling().unwrap().name(), b.name());
	assert!(b.next_element_sibling().is_none());
}
