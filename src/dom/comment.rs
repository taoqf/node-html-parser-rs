#[derive(Debug, Clone)]
pub struct CommentNode {
	pub text: String,
	pub range: Option<(usize, usize)>,
}

impl CommentNode {
	pub fn new(text: String) -> Self {
		Self { text, range: None }
	}
	pub fn with_range(text: String, start: usize, end: usize) -> Self {
		Self {
			text,
			range: Some((start, end)),
		}
	}
	pub fn range(&self) -> Option<(usize, usize)> {
		self.range
	}
}
