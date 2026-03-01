use node_html_parser::parse;

#[test]
fn issue_70_attribute_with_colon_and_underscore() {
	let html1 = "\n\n<!doctype html>\n<html class=\"no-js\" lang=\"en\">\n\n<head> \n\n<meta property=\"og:type\" content=\"product\" />\n</head></html>";
	let root1 = parse(html1);
	let meta = root1.query_selector("meta").unwrap();
	assert_eq!(meta.get_attr("property"), Some("og:type"));

	let html2 = "<button type=\"submit\" name=\"add-to-cart\" value=\"12121\" data-product_id=\"12121\" data-quantity=\"1\" class=\"some_add_to_cart_class other_add_to_cart_class third_add_to_cart_class random_other_class  button alt\">My button</button>\n";
	let root2 = parse(html2);
	let btn = root2.query_selector("button").unwrap();
	assert_eq!(btn.get_attr("data-product_id"), Some("12121"));
}
