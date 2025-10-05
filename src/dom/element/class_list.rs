use super::main::HTMLElement;

impl HTMLElement {
	fn ensure_class_cache(&mut self) {
		if self.class_cache.is_none() {
			let list = self.get_attribute("class").unwrap_or_default();
			let tokens: Vec<String> = list
				.split_whitespace()
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string())
				.collect();
			self.class_cache = Some(tokens);
		}
	}
	pub fn class_list(&mut self) -> Vec<String> {
		self.ensure_class_cache();
		self.class_cache.clone().unwrap()
	}
	pub fn class_list_view(&self) -> Vec<&str> {
		self.get_attr("class")
			.map(|s| s.split_whitespace().collect())
			.unwrap_or_else(Vec::new)
	}
	pub fn class_list_contains(&mut self, token: &str) -> bool {
		self.ensure_class_cache();
		self.class_cache
			.as_ref()
			.unwrap()
			.iter()
			.any(|c| c == token)
	}
	pub fn class_list_add(&mut self, token: &str) {
		self.ensure_class_cache();
		if let Some(v) = self.class_cache.as_mut() {
			if v.iter().any(|c| c == token) {
				return;
			}
			let mut new_tokens = v.clone();
			new_tokens.push(token.to_string());
			let new_val = new_tokens.join(" ");
			self.set_attributes(&[("class".to_string(), new_val)]);
			self.class_cache = Some(new_tokens);
		}
	}
	pub fn class_list_remove(&mut self, token: &str) {
		self.ensure_class_cache();
		if let Some(v) = self.class_cache.as_mut() {
			if !v.iter().any(|c| c == token) {
				return;
			}
			let new_tokens: Vec<String> =
				v.iter().filter(|c| c.as_str() != token).cloned().collect();
			let new_val = new_tokens.join(" ");
			self.set_attributes(&[("class".to_string(), new_val.clone())]);
			self.class_cache = if new_val.is_empty() {
				None
			} else {
				Some(new_tokens)
			};
		}
	}
	pub fn class_list_toggle(&mut self, token: &str) {
		if self.class_list_contains(token) {
			self.class_list_remove(token);
		} else {
			self.class_list_add(token);
		}
	}
	pub fn class_list_replace(&mut self, old: &str, new: &str) {
		self.ensure_class_cache();
		if let Some(v) = self.class_cache.as_mut() {
			if !v.iter().any(|c| c == old) {
				return;
			}
			let mut new_tokens: Vec<String> = Vec::with_capacity(v.len());
			for c in v.iter() {
				if c == old {
					new_tokens.push(new.to_string());
				} else {
					new_tokens.push(c.clone());
				}
			}
			let new_val = new_tokens.join(" ");
			self.set_attributes(&[("class".to_string(), new_val.clone())]);
			self.class_cache = Some(new_tokens);
		}
	}

	fn sync_class_attr(&mut self) {
		if let Some(cache) = &self.class_cache {
			let joined = cache.join(" ");
			self.set_attr("class", &joined);
		}
	}
}
