use node_html_parser::{valid, Options};

#[test]
fn issue_227_valid_edge_cases() {
	let opts = Options::default();
	assert!(!valid("<p abc</p>", &opts));
	assert!(!valid("<div<p abc</p></span>", &opts));
	assert!(valid("<div><p abc</p></div>", &opts));
	assert!(valid("<div<p abc</a></div>", &opts));
	assert!(valid("@#><p", &opts));
	assert!(valid("<<>", &opts));
}
