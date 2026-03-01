//! Public API entry points for HTML parsing.

use crate::dom::element::HTMLElement;

use super::core_parser::parse_with_options;
use super::types::Options;

/// Parse HTML string with default options.
///
/// This is the main entry point for parsing HTML. It uses sensible defaults
/// and returns the root element containing the parsed DOM tree.
///
/// # Arguments
///
/// * `input` - The HTML string to parse
///
/// # Returns
///
/// A boxed HTMLElement representing the root of the parsed DOM tree
///
/// # Examples
///
/// ```rust
/// use node_html_parser::parse;
///
/// let root = parse("<div>Hello world</div>");
/// assert_eq!(root.children.len(), 1);
/// ```
pub fn parse(input: &str) -> Box<HTMLElement> {
	parse_with_options(input, &Options::default())
}
