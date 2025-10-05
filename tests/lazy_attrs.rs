use node_html_parser::{parse, HTMLElement};

fn collect_attr_keys(el: &HTMLElement) -> Vec<String> {
	el.attrs.iter().map(|(k, _)| k.clone()).collect()
}

#[test]
fn lazy_attribute_parsing_basic() {
	// Mix of quoted, single-quoted, boolean and unquoted forms
	let html = "<div id='root'><span id='a' class=cls data-x=1 title='T' data-flag attrNoValue style=\"color:red\" foo-bar='xyz'></span></div>";
	let root = parse(html);
	let div = root.first_element_child().expect("div");
	let span = div.first_element_child().expect("span");

	// Initial state: only id & class should be eagerly decoded (plus possibly class even if unquoted)
	let keys_before = collect_attr_keys(span);
	assert!(keys_before.contains(&"id".to_string()));
	assert!(keys_before.contains(&"class".to_string()));
	assert!(!keys_before.contains(&"data-x".to_string()));
	assert!(!keys_before.contains(&"title".to_string()));
	assert!(!keys_before.contains(&"style".to_string()));
	assert!(!keys_before.contains(&"foo-bar".to_string()));

	// Accessing a lazy attribute triggers full parse
	assert_eq!(span.get_attr("data-x"), Some("1"));
	assert_eq!(span.get_attr("title"), Some("T"));
	assert_eq!(span.get_attr("style"), Some("color:red"));
	assert_eq!(span.get_attr("foo-bar"), Some("xyz"));
	// Boolean / no-value attributes should appear with empty value string
	assert_eq!(span.get_attr("data-flag"), Some(""));
	assert_eq!(span.get_attr("attrNoValue"), Some(""));

	let keys_after = collect_attr_keys(span);
	// Now all referenced attributes should be present
	for k in [
		"id",
		"class",
		"data-x",
		"title",
		"data-flag",
		"attrnovalue",
		"style",
		"foo-bar",
	] {
		assert!(
			keys_after.contains(&k.to_string()),
			"expected key '{}' after lazy load",
			k
		);
	}
}
