use node_html_parser::parse;

#[test]
fn escapes_double_quotes_set_attribute() {
	let root = parse("<div></div>");
	let mut div = root.query_selector("div").unwrap().clone();
	div.set_attribute("foo", "[{\"bar\":\"baz\"}]");
	let out = div.to_string();
	assert_eq!(
		out,
		"<div foo=\"[{&quot;bar&quot;:&quot;baz&quot;}]\"></div>"
	);
}

#[test]
fn escapes_double_quotes_set_attributes() {
	let root = parse("<div></div>");
	let mut div = root.query_selector("div").unwrap().clone();
	div.set_attributes(&[("foo".to_string(), "[{\"bar\":\"baz\"}]".to_string())]);
	let out = div.to_string();
	assert_eq!(
		out,
		"<div foo=\"[{&quot;bar&quot;:&quot;baz&quot;}]\"></div>"
	);
}

#[test]
fn parses_attributes_with_quot_entities() {
	let root = parse("<div foo=\"[{&quot;bar&quot;:&quot;baz&quot;}]\"></div>");
	let mut div = root.query_selector("div").unwrap().clone();
	assert_eq!(div.get_attr("foo").unwrap(), "[{\"bar\":\"baz\"}]");
	let attrs = div.attributes();
	assert_eq!(attrs.get("foo").unwrap(), "[{\"bar\":\"baz\"}]");
}
