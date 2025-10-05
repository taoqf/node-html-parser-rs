#[derive(Debug, Clone)]
pub struct TextNode {
	pub raw: String,
	pub range: Option<(usize, usize)>,
	trimmed_raw_cache: Option<String>,
	trimmed_txt_cache: Option<String>,
}

impl TextNode {
	pub fn new(raw: String) -> Self {
		Self {
			raw,
			range: None,
			trimmed_raw_cache: None,
			trimmed_txt_cache: None,
		}
	}
	pub fn with_range(raw: String, start: usize, end: usize) -> Self {
		Self {
			raw,
			range: Some((start, end)),
			trimmed_raw_cache: None,
			trimmed_txt_cache: None,
		}
	}
	pub fn range(&self) -> Option<(usize, usize)> {
		self.range
	}
	fn invalidate(&mut self) {
		self.trimmed_raw_cache = None;
		self.trimmed_txt_cache = None;
	}
	pub fn set_raw(&mut self, v: String) {
		self.raw = v;
		self.invalidate();
	}
	fn trim_alg(text: &str) -> String {
		if text.is_empty() {
			return String::new();
		}
		let bytes = text.as_bytes();
		let mut start = 0usize;
		let mut end = bytes.len() - 1;
		while start < bytes.len() {
			if !bytes[start].is_ascii_whitespace() {
				break;
			}
			start += 1;
		}
		while end > start {
			if !bytes[end].is_ascii_whitespace() {
				break;
			}
			end -= 1;
		}
		let has_leading = start > 0;
		let has_trailing = end < bytes.len() - 1;
		format!(
			"{}{}{}",
			if has_leading { " " } else { "" },
			&text[start..=end],
			if has_trailing { " " } else { "" }
		)
	}
	pub fn trimmed_raw_text(&mut self) -> &str {
		if self.trimmed_raw_cache.is_none() {
			self.trimmed_raw_cache = Some(Self::trim_alg(&self.raw));
		}
		self.trimmed_raw_cache.as_ref().unwrap()
	}
	pub fn trimmed_text(&mut self) -> &str {
		if self.trimmed_txt_cache.is_none() {
			let dec = html_escape::decode_html_entities(&self.raw).to_string();
			self.trimmed_txt_cache = Some(Self::trim_alg(&dec));
		}
		self.trimmed_txt_cache.as_ref().unwrap()
	}
	pub fn is_whitespace(&self) -> bool {
		regex::Regex::new(r"^(?:\s|&nbsp;)*$")
			.unwrap()
			.is_match(&self.raw)
	}
	pub fn text(&self) -> String {
		html_escape::decode_html_entities(&self.raw).to_string()
	}
	pub fn raw_text(&self) -> &str {
		&self.raw
	}
	pub fn decoded_text(&self) -> String {
		self.text()
	}
}
