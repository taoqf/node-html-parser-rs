use super::main::HTMLElement;
use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

// 缓存属性解析相关的正则表达式
static ATTR_PARSE_REGEX: OnceLock<Regex> = OnceLock::new();

impl HTMLElement {
	pub fn attrs_lower_decoded(&mut self) -> HashMap<String, String> {
		self.ensure_lower_decoded();
		self.cache_lower_decoded.clone().unwrap_or_default()
	}

	pub fn set_attributes(&mut self, attributes: &[(String, String)]) {
		// 重建 raw_attrs 与 attrs（attrs 的 key 需小写且解码，这里假设传入 value 已为未转义文本，与 JS 行为接近）
		self.attrs = attributes
			.iter()
			.map(|(k, v)| (k.to_lowercase(), v.clone()))
			.collect();
		self.raw_attrs = attributes
			.iter()
			.map(|(k, v)| {
				// JS setAttributes: treats raw value 'null' OR '""' OR empty as boolean attribute (only name)
				if v.is_empty() || v == "null" || v == "\"\"" {
					k.clone()
				} else {
					format!("{}={}", k, quote_attribute(v))
				}
			})
			.collect::<Vec<_>>()
			.join(" ");
		self.cache_raw_map = None;
		self.cache_lower_decoded = None;
		// 更新 id / class cache
		if let Some((_, idv)) = self.attrs.iter().find(|(kk, _)| kk == "id") {
			self.id = idv.clone();
		}
		if self.attrs.iter().any(|(kk, _)| kk == "class") {
			self.class_cache = None;
		}
	}
	pub fn remove_attribute(&mut self, key: &str) {
		self.build_raw_cache();
		let mut raw_map = self.cache_raw_map.take().unwrap_or_default();
		let target = key.to_lowercase();
		raw_map.retain(|k, _| k.to_lowercase() != target);
		// sync structured attrs vector
		self.attrs.retain(|(kk, _)| kk != &target);
		self.raw_attrs = raw_map
			.iter()
			.map(|(k, v)| {
				if v.is_empty() {
					k.clone()
				} else {
					format!("{}={}", k, quote_attribute(v))
				}
			})
			.collect::<Vec<_>>()
			.join(" ");
		self.cache_raw_map = None;
		self.cache_lower_decoded = None;
		if target == "id" {
			self.id.clear();
		}
		if target == "class" {
			self.class_cache = None;
		}
		self.attrs_complete = true; // attrs now reflect full set
		self.attrs_modified = true; // Mark attributes as modified
	}

	pub fn get_attr(&self, key: &str) -> Option<&str> {
		// First try already parsed attributes
		let k = key.to_lowercase();
		if let Some(found) = self.attrs.iter().find(|(kk, _)| *kk == k) {
			return Some(found.1.as_str());
		}

		// If not found and attrs not complete, we need to ensure parsing
		if !self.attrs_complete && !self.raw_attrs.is_empty() {
			// Use unsafe to trigger ensure_all_attrs on self
			let mut_ptr = self as *const HTMLElement as *mut HTMLElement;
			unsafe {
				(*mut_ptr).ensure_all_attrs();
				// Now search again in the updated attrs
				return (*mut_ptr)
					.attrs
					.iter()
					.find(|(kk, _)| *kk == k)
					.map(|(_, v)| v.as_str());
			}
		}

		None
	}
	pub fn has_attr(&self, key: &str) -> bool {
		self.get_attr(key).is_some()
	}

	pub fn set_attr(&mut self, key: &str, val: &str) {
		let k = key.to_lowercase();
		if let Some(kv) = self.attrs.iter_mut().find(|(kk, _)| *kk == k) {
			kv.1 = val.to_string();
		} else {
			self.attrs.push((k, val.to_string()));
		}
		self.rebuild_raw_attrs();
		self.cache_raw_map = None;
		self.cache_lower_decoded = None;
		if key.eq_ignore_ascii_case("id") {
			self.id = val.to_string();
		}
	}
	pub fn remove_attr(&mut self, key: &str) {
		let k = key.to_lowercase();
		self.attrs.retain(|(kk, _)| *kk != k);
		self.rebuild_raw_attrs();
		self.cache_raw_map = None;
		self.cache_lower_decoded = None;
		if k == "id" {
			self.id.clear();
		}
	}
	/// Convenience: remove the id attribute (safe wrapper for tests parity with JS removeAttribute('id'))
	pub fn remove_id(&mut self) {
		self.remove_attribute("id");
	}
	/// Convenience: set id attribute (safe wrapper to avoid direct raw mutation in tests)
	pub fn set_id(&mut self, id: &str) {
		self.set_attribute("id", id);
	}
	pub(super) fn rebuild_raw_attrs(&mut self) {
		// 保持原有顺序，使用与 JS Quote 逻辑更接近的方式（参见 nodes/html.ts quoteAttribute）
		fn quote_attr(src: &str) -> String {
			if src.is_empty() || src == "null" {
				return src.to_string();
			}
			// 先替换双引号
			let replaced = src.replace('"', "&quot;");
			// 模拟 JS: JSON.stringify 然后还原制表/换行/回车并移除反斜杠
			let jsoned =
				serde_json::to_string(&replaced).unwrap_or_else(|_| format!("\"{}\"", replaced));
			// jsoned 形如 "..."，去掉外层引号后处理内部转义
			let inner = jsoned.trim_matches('"');
			let inner = inner
				.replace("\\t", "\t")
				.replace("\\n", "\n")
				.replace("\\r", "\r")
				.replace('\\', "");
			format!("\"{}\"", inner)
		}
		self.raw_attrs = self
			.attrs
			.iter()
			.map(|(k, v)| {
				if v.is_empty() {
					k.clone()
				} else {
					format!("{}={}", k, quote_attr(v))
				}
			})
			.collect::<Vec<_>>()
			.join(" ");
	}

	pub fn attributes(&mut self) -> std::collections::HashMap<String, String> {
		// JS: Element.attributes preserves original attribute name casing/order (first occurrence) while returning decoded values.
		// We approximate with a HashMap (order not guaranteed) but keep original key casing from raw parsing.
		self.build_raw_cache();
		let mut out = std::collections::HashMap::new();
		if let Some(raw) = &self.cache_raw_map {
			for (orig_k, raw_v) in raw.iter() {
				let decoded = html_escape::decode_html_entities(raw_v).to_string();
				// Insert only if absent (first occurrence wins) – raw_map already keeps first, so direct insert.
				out.insert(orig_k.clone(), decoded);
			}
		}
		out
	}
	pub fn raw_attributes(&mut self) -> HashMap<String, String> {
		self.build_raw_cache();
		self.cache_raw_map.clone().unwrap_or_default()
	}
	/// Read-only snapshot of the original raw attribute string (public accessor for tests like issue 136)
	pub fn raw_attrs_str(&self) -> &str {
		&self.raw_attrs
	}

	pub fn get_attribute(&mut self, key: &str) -> Option<String> {
		self.ensure_lower_decoded();
		self.cache_lower_decoded
			.as_ref()
			.unwrap()
			.get(&key.to_lowercase())
			.cloned()
	}

	pub fn set_attribute(&mut self, key: &str, value: &str) {
		// Update raw_attrs string representation, preserving original attribute order
		let quoted_value = if value.is_empty() {
			None
		} else {
			Some(quote_attribute(value))
		};

		if self.raw_attrs.is_empty() {
			if let Some(qv) = quoted_value {
				self.raw_attrs = format!("{}={}", key, qv);
			} else {
				self.raw_attrs = key.to_string();
			}
		} else {
			// Parse existing attributes to preserve order
			let re = ATTR_PARSE_REGEX.get_or_init(|| {
				regex::Regex::new(
					r#"([a-zA-Z()\[\]#@$.?:][a-zA-Z0-9-._:()\[\]#]*)(?:\s*=\s*((?:'[^']*')|(?:"[^"]*")|\S+))?"#,
				)
				.unwrap()
			});

			let mut result_attrs = Vec::new();
			let mut found = false;

			for cap in re.captures_iter(&self.raw_attrs) {
				let existing_key = cap.get(1).unwrap().as_str();
				if existing_key.eq_ignore_ascii_case(key) {
					// Replace this attribute, preserve original case
					if let Some(qv) = &quoted_value {
						result_attrs.push(format!("{}={}", existing_key, qv));
					} else {
						result_attrs.push(existing_key.to_string());
					}
					found = true;
				} else {
					// Keep existing attribute as-is
					let existing_val = cap.get(2).map(|m| m.as_str()).unwrap_or("");
					if existing_val.is_empty() {
						result_attrs.push(existing_key.to_string());
					} else {
						result_attrs.push(format!("{}={}", existing_key, existing_val));
					}
				}
			}

			// If not found, add at the end
			if !found {
				if let Some(qv) = quoted_value {
					result_attrs.push(format!("{}={}", key, qv));
				} else {
					result_attrs.push(key.to_string());
				}
			}

			self.raw_attrs = result_attrs.join(" ");
		}

		// Update structured attrs with decoded value
		self.ensure_all_attrs();
		let lk = key.to_lowercase();
		let decoded_val = html_escape::decode_html_entities(value).to_string();
		if let Some(kv) = self.attrs.iter_mut().find(|(k, _)| *k == lk) {
			kv.1 = decoded_val;
		} else {
			self.attrs.push((lk, decoded_val));
		}

		// Clear caches to force rebuild
		self.cache_raw_map = None;
		self.cache_lower_decoded = None;
		self.attrs_complete = true;
		self.attrs_modified = true; // Mark attributes as modified

		// Update element-specific caches
		if key.eq_ignore_ascii_case("id") {
			self.id = value.to_string();
		}
		if key.eq_ignore_ascii_case("class") {
			self.class_cache = None;
		}
	}

	pub fn has_attribute(&mut self, key: &str) -> bool {
		self.ensure_lower_decoded();
		self.cache_lower_decoded
			.as_ref()
			.unwrap()
			.contains_key(&key.to_lowercase())
	}

	pub(crate) fn ensure_all_attrs(&mut self) {
		if self.attrs_complete {
			return;
		}

		// Clear existing attrs and rebuild from raw_attrs string
		self.attrs.clear();
		self.build_raw_cache();
		if let Some(ref raw_map) = self.cache_raw_map {
			for (key, value) in raw_map.iter() {
				let decoded_val = html_escape::decode_html_entities(value).to_string();
				self.attrs.push((key.to_lowercase(), decoded_val));
			}
		}

		self.attrs_complete = true;
	}
	fn build_raw_cache(&mut self) {
		if self.cache_raw_map.is_some() {
			return;
		}

		let mut map = HashMap::new();
		if !self.raw_attrs.is_empty() {
			let re = ATTR_PARSE_REGEX.get_or_init(|| {
				regex::Regex::new(
					r#"([a-zA-Z()\[\]#@$.?:][a-zA-Z0-9-._:()\[\]#]*)(?:\s*=\s*((?:'[^']*')|(?:"[^"]*")|\S+))?"#,
				)
				.unwrap()
			});
			for cap in re.captures_iter(&self.raw_attrs) {
				let key = cap.get(1).unwrap().as_str();
				let mut val = cap.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
				if !val.is_empty() {
					if (val.starts_with('\"') && val.ends_with('\"'))
						|| (val.starts_with('\'') && val.ends_with('\''))
					{
						val = val[1..val.len() - 1].to_string();
					}
				}
				// only first occurrence kept (JS behavior)
				map.entry(key.to_string()).or_insert(val);
			}
		}
		self.cache_raw_map = Some(map);
	}

	fn ensure_lower_decoded(&mut self) {
		if self.cache_lower_decoded.is_some() {
			return;
		}

		self.build_raw_cache();
		let mut lower_decoded = HashMap::new();

		if let Some(ref raw_map) = self.cache_raw_map {
			for (key, value) in raw_map.iter() {
				let decoded_val = html_escape::decode_html_entities(value).to_string();
				let lower_key = key.to_lowercase();
				lower_decoded.insert(lower_key, decoded_val);
			}
		}

		self.cache_lower_decoded = Some(lower_decoded);
	}
}

fn quote_attribute(val: &str) -> String {
	if val.is_empty() {
		return val.to_string();
	}
	let replaced = val.replace('"', "&quot;");
	let jsoned = serde_json::to_string(&replaced).unwrap_or_else(|_| format!("\"{}\"", replaced));
	let inner = jsoned.trim_matches('"');
	let inner = inner
		.replace("\\t", "\t")
		.replace("\\n", "\n")
		.replace("\\r", "\r")
		.replace('\\', "");
	format!("\"{}\"", inner)
}
