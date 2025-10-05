use node_html_parser::{parse, parse_with_options, Options};

#[test]
fn pre_block_as_raw_when_enabled() {
	// Minimal version of code sample; focus on raw preservation.
	let html = "<div class=\"wrapper\"><pre class=\"highlight\"><code><span class=\"k\">print</span>(x)<br></code></pre></div>";
	let opts = Options::default(); // pre raw by default
	let root = parse_with_options(html, &opts);
	let pre = root.query_selector("pre.highlight").unwrap();
	// Raw mode: inner_html should be the literal code markup (unchanged), and spans should NOT be parsed as child elements of <pre> (only inside raw text)
	assert!(pre
		.inner_html()
		.starts_with("<code><span class=\"k\">print"));
	// In raw mode, there should be exactly one child text node (no span element under pre)
	assert!(
		pre.query_selector("span.k").is_none(),
		"span should not be parsed as element inside raw pre"
	);
}

#[test]
fn pre_raw_only_when_specified() {
	let html = "<outer><pre class=\"highlight\"><code><span class=\"k\">print</span> 1<br></code></pre></outer>";
	let mut opts = Options::default();
	opts.block_text_elements.clear();
	opts.block_text_elements.insert("pre".into(), true); // only pre
	let root = parse_with_options(html, &opts);
	let pre = root.query_selector("pre.highlight").unwrap();
	assert!(pre
		.inner_html()
		.contains("<code><span class=\"k\">print</span> 1<br></code>"));
	assert!(pre.query_selector("span.k").is_none());
}

#[test]
fn pre_not_raw_when_disabled() {
	let html = "<div><pre class=\"highlight\"><code><span class=\"k\">print</span>(x)<br></code></pre></div>";
	let mut opts = Options::default();
	opts.block_text_elements.clear(); // disable raw handling for all
	let root = parse_with_options(html, &opts);
	let pre = root.query_selector("pre.highlight").unwrap();
	let code = pre.query_selector("code").unwrap();
	let spans = code.query_selector_all("span.k");
	assert_eq!(
		spans.len(),
		1,
		"span should be parsed as element when pre not raw"
	);
}

#[test]
fn pre_partial_name_not_matched() {
	let doc_root = parse("<premises><color>Red</color></premises>");
	// Ensure 'premises' not treated as raw block (so its child <color> is a parsed element)
	let color = doc_root.query_selector("color").unwrap();
	assert_eq!(color.name().to_ascii_uppercase(), "COLOR");
}

#[test]
fn pre_complex_multiline_sample() {
	// Reintroduce a more realistic code highlighting block similar to original JS test, with correct quoting (no backslash-escaped quotes inside raw string).
	let html = r#"
<div class="language-python highlighter-rouge">
    <div class="highlight">
        <pre class="highlight"><code><span class="k">print</span><span class="p">(</span><span class="s">'hello'</span><span class="p">)</span><br><span class="n">i</span> <span class="o">=</span> <span class="n">i</span> <span class="o">+</span> <span class="mi">1</span><br></code></pre>
    </div>
</div>
"#;
	// Default options (pre raw)
	let root = parse_with_options(html, &Options::default());
	let pre = root.query_selector("pre.highlight").unwrap();
	let inner = pre.inner_html();
	assert!(
		inner.contains("<code><span class=\"k\">print</span>"),
		"inner lost spans (raw mode) => {inner}"
	);
	// Ensure raw: spans not individually accessible as elements under pre
	assert!(
		pre.query_selector("span.k").is_none(),
		"raw pre should not parse nested spans as DOM elements"
	);
	// Outer div class should remain intact (verifies attribute parsing unaffected)
	let root_mut = root; // take ownership to allow mutable borrows
	let outer = root_mut.query_selector("div.language-python").unwrap();
	let mut outer_ref = outer.clone(); // clone node to get owned mutable instance
	let class_val = outer_ref.get_attribute("class").unwrap();
	assert_eq!(class_val, "language-python highlighter-rouge");
}
