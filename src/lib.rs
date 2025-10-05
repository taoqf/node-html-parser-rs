pub mod css_select;
pub(crate) mod dom;
// Re-export HTMLElement for tests needing direct construction parity with JS (issue #112)
pub(crate) mod parser;
// Re-export experimental selector compile utilities (explicit opt-in for external tests)
pub use css_select::compile_experimental;
pub use css_select::types::{HtmlAdapter as CssHtmlAdapter, Options as CssSelectOptions};
pub use dom::comment::CommentNode;
pub use dom::element::main::HTMLElement;
pub use dom::node::Node;
pub use dom::node_type::NodeType;
pub use dom::text::TextNode;
pub use parser::{parse, parse_with_options, valid, Options};
