use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct VoidTagOptions {
	pub add_closing_slash: bool,
	pub tags: Option<Vec<String>>, // custom tag list (case-insensitive)
}

impl Default for VoidTagOptions {
	fn default() -> Self {
		Self {
			add_closing_slash: false,
			tags: None,
		}
	}
}

#[derive(Debug, Clone)]
pub struct VoidTag {
	// add_closing_slash: bool,
	set: HashSet<String>, // store both lower & upper for quick checks
}

impl VoidTag {
	pub fn new(opts: &VoidTagOptions) -> Self {
		let base: Vec<String> = if let Some(custom) = &opts.tags {
			custom.clone()
		} else {
			vec![
				"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
				"param", "source", "track", "wbr",
			]
			.into_iter()
			.map(|s| s.to_string())
			.collect()
		};
		let mut set = HashSet::new();
		for t in base.iter() {
			set.insert(t.to_lowercase());
			set.insert(t.to_uppercase());
		}
		Self {
			// add_closing_slash: opts.add_closing_slash,
			set,
		}
	}
	pub fn is_void(&self, tag: &str) -> bool {
		self.set.contains(tag)
	}

	// pub fn format(&self, tag: &str, attrs: &str, inner: &str) -> String {
	// 	if self.is_void(tag) {
	// 		if self.add_closing_slash {
	// 			format!("<{}{} />", tag, attrs)
	// 		} else {
	// 			format!("<{}{}>", tag, attrs)
	// 		}
	// 	} else {
	// 		format!("<{}{}>{}</{}>", tag, attrs, inner, tag)
	// 	}
	// }
}
