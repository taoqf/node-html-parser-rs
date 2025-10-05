pub(crate) mod atoms;

fn main() {
	let html = "<div class='greet'>Hello <b>Rust</b>!</div>";
	let root = node_html_parser::parse(html);
	println!("Parsed children count: {}", root.children.len());
	println!("Output: {}", root.inner_html());
}
