use node_html_parser::{parse, valid, Options};
use std::fs;

// Ported from js/tests/valid.js

fn load_asset(name: &str) -> String {
	let path = format!("tests/assets/html/{}", name);
	fs::read_to_string(path).expect("asset file not found")
}

fn default_opts() -> Options {
	Options::default()
}

#[test]
fn parse_with_validation_double_p_open_no_error() {
	let html = "<p><p></p>"; // js comment: fixes to <p></p><p></p>
	let ok = valid(html, &default_opts());
	assert!(ok, "expected valid=true");
}

#[test]
fn parse_with_validation_p_self_closing_nested() {
	let html = "<p><p/></p>"; // js: treated as <p><p></p></p>
	let ok = valid(html, &default_opts());
	assert!(ok);
}

#[test]
fn parse_with_validation_p_h3_mismatch() {
	let html = "<p><h3></p>";
	let ok = valid(html, &default_opts());
	assert!(!ok, "expected invalid");
}

#[test]
fn hillcrestpartyrentals_invalid_unclosed_p() {
	let data = load_asset("hillcrestpartyrentals.html");
	let ok = valid(&data, &default_opts());
	assert!(!ok, "expected invalid for hillcrestpartyrentals");
}

#[test]
fn google_valid() {
	let data = load_asset("google.html");
	let ok = valid(&data, &default_opts());
	assert!(ok, "google.html should be valid");
}

#[test]
fn gmail_valid() {
	let data = load_asset("gmail.html");
	let ok = valid(&data, &default_opts());
	assert!(ok, "gmail.html should be valid");
}

#[test]
fn ffmpeg_invalid_extra_div() {
	let data = load_asset("ffmpeg.html");
	let ok = valid(&data, &default_opts());
	assert!(!ok, "ffmpeg.html should be invalid");
}

#[test]
fn fix_div_h3_div_to_div_h3_closed() {
	let html = "<div data-id=1><h3 data-id=2><h3><div>";
	let ok = valid(html, &default_opts());
	assert!(!ok, "expected invalid before fix");
	let root = parse(html);
	assert_eq!(
		root.to_string(),
		r#"<div data-id="1"><h3 data-id="2"></h3></div>"#
	);
}

#[test]
fn fix_div_h3_span_chain() {
	let html = "<div><h3><h3><span><span><div>";
	let ok = valid(html, &default_opts());
	assert!(!ok);
	let root = parse(html);
	assert_eq!(root.to_string(), "<div><h3></h3><span></span></div>");
}

#[test]
fn gmail_corrupted_should_be_invalid() {
	let mut data = load_asset("gmail.html");
	data = data.replace("</", "<");
	let ok = valid(&data, &default_opts());
	assert!(!ok);
}

#[test]
fn nice_corrupted_should_be_invalid() {
	let mut data = load_asset("nice.html");
	data = data.replace("</", "<");
	let ok = valid(&data, &default_opts());
	assert!(!ok);
}
