#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
	Element = 1,
	Text = 3,
	Comment = 8,
}
