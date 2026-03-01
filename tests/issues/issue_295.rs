use node_html_parser::{parse_with_options, Options};

#[test]
fn issue_295_valid_nesting_kept() {
    let root = parse_with_options("<div>foo<div>bar</div></div>", &Options::default());
    assert_eq!(root.to_string(), "<div>foo<div>bar</div></div>");
}

#[test]
fn issue_295_preserve_p_nested_inside_p() {
    let mut opts = Options::default();
    opts.preserve_tag_nesting = true;
    let root = parse_with_options("<p>foo<p>bar</p></p>", &opts);
    assert_eq!(root.to_string(), "<p>foo<p>bar</p></p>");
}

#[test]
fn issue_295_ul_inside_p_kept() {
    let root = parse_with_options("<p>foo<ul><li>bar</li></ul></p>", &Options::default());
    assert_eq!(root.to_string(), "<p>foo<ul><li>bar</li></ul></p>");
}
