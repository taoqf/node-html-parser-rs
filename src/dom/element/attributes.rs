use super::main::HTMLElement;
use regex::Regex;
use std::collections::HashMap;

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
		self.ensure_raw_attributes();
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
	}

	pub fn get_attr(&self, key: &str) -> Option<&str> {
		// 需要可变以便确保延迟属性完成解析
		let k = key.to_lowercase();
		let mut_ptr = self as *const HTMLElement as *mut HTMLElement; // unsafe 以允许内部完成解析
		unsafe {
			(*mut_ptr).ensure_all_attrs();
		}
		self.attrs
			.iter()
			.find(|(kk, _)| *kk == k)
			.map(|(_, v)| v.as_str())
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
	pub fn remove_id(&mut self) { self.remove_attribute("id"); }
	/// Convenience: set id attribute (safe wrapper to avoid direct raw mutation in tests)
	pub fn set_id(&mut self, id: &str) { self.set_attribute("id", id); }
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

	// --- JS style attribute parsing (rawAttributes) ---
	fn ensure_raw_attributes(&mut self) {
		if self.cache_raw_map.is_some() {
			return;
		}
		let mut map = HashMap::new();
		if !self.raw_attrs.is_empty() {
			let re = regex::Regex::new(
				r#"([a-zA-Z()\[\]#@$.?:][a-zA-Z0-9-._:()\[\]#]*)(?:\s*=\s*((?:'[^']*')|(?:"[^"]*")|\S+))?"#,
			)
			.unwrap();
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

	pub fn attributes(&mut self) -> std::collections::HashMap<String, String> {
		// JS: Element.attributes preserves original attribute name casing/order (first occurrence) while returning decoded values.
		// We approximate with a HashMap (order not guaranteed) but keep original key casing from raw parsing.
		self.ensure_raw_attributes();
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
		self.ensure_raw_attributes();
		self.cache_raw_map.clone().unwrap_or_default()
	}
	/// Read-only snapshot of the original raw attribute string (public accessor for tests like issue 136)
	pub fn raw_attrs_str(&self) -> &str { &self.raw_attrs }

	pub fn get_attribute(&mut self, key: &str) -> Option<String> {
		self.ensure_lower_decoded();
		self.cache_lower_decoded
			.as_ref()
			.unwrap()
			.get(&key.to_lowercase())
			.cloned()
	}

	pub fn set_attribute(&mut self, key: &str, value: &str) {
		// JS preserves original attribute order; new attributes appended.
		// Strategy: re-parse current raw_attrs to ordered vector of keys, update/append target, rebuild string.
		self.ensure_raw_attributes();
		let raw_snapshot = self.raw_attrs.clone();
		let re = regex::Regex::new(
			r#"([a-zA-Z()\[\]#@$.?:][a-zA-Z0-9-._:()\[\]#]*)(?:\s*=\s*((?:'[^']*')|(?:\"[^\"]*\")|[^\s>]+))?"#,
		).unwrap();
		let mut order: Vec<String> = Vec::new();
		let mut seen_ci: Vec<String> = Vec::new();
		for cap in re.captures_iter(&raw_snapshot) {
			let k = cap.get(1).unwrap().as_str().to_string();
			let k_ci = k.to_lowercase();
			if !seen_ci.iter().any(|x| x == &k_ci) {
				order.push(k.clone());
				seen_ci.push(k_ci);
			}
		}
		// Build map (original casing -> raw value) from existing cache_raw_map (original keys) to preserve first casing.
		let mut new_map: std::collections::HashMap<String, String> =
			std::collections::HashMap::new();
		if let Some(raw) = &self.cache_raw_map {
			for (k, v) in raw.iter() {
				new_map.insert(k.clone(), v.clone());
			}
		}
		// Determine if key exists case-insensitively; if so update that original key.
		let mut target_original: Option<String> = None;
		for k in order.iter() {
			if k.eq_ignore_ascii_case(key) {
				target_original = Some(k.clone());
				break;
			}
		}
		if let Some(orig) = target_original {
			new_map.insert(orig, value.to_string());
		} else {
			order.push(key.to_string());
			new_map.insert(key.to_string(), value.to_string());
		}
		// Reconstruct raw_attrs following order vector.
		let mut parts = Vec::with_capacity(order.len());
		for k in &order {
			if let Some(v) = new_map.get(k) {
				if v.is_empty() {
					parts.push(k.clone());
				} else {
					parts.push(format!("{}={}", k, quote_attribute(v)));
				}
			}
		}
		self.raw_attrs = parts.join(" ");
		self.cache_raw_map = None; // force rebuild
		self.cache_lower_decoded = None;
		// sync structured attrs (store lowercase key, decoded value as get_attr expects)
		let lk = key.to_lowercase();
		let decoded_val = html_escape::decode_html_entities(value).to_string();
		if let Some(kv) = self.attrs.iter_mut().find(|(k, _)| *k == lk) {
			kv.1 = decoded_val.clone();
		} else {
			self.attrs.push((lk.clone(), decoded_val.clone()));
		}
		self.attrs_complete = true;
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
		if self.raw_attrs.is_empty() {
			self.attrs_complete = true;
			return;
		}
		static ATTR_RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
		let re = ATTR_RE.get_or_init(|| {
			regex::Regex::new(
				r#"([a-zA-Z()\[\]#@$.?:][a-zA-Z0-9-._:()\[\]#]*)(?:\s*=\s*((?:'[^']*')|(?:"[^"]*")|\S+))?"#,
			)
			.unwrap()
		});
		for cap in re.captures_iter(&self.raw_attrs) {
			let key = cap.get(1).unwrap().as_str();
			let val = cap.get(2).map(|m| m.as_str()).unwrap_or("");
			let unquoted = if val.starts_with('"') || val.starts_with('\'') {
				&val[1..val.len() - 1]
			} else {
				val
			};
			let lk = key.to_lowercase();
			if !self.attrs.iter().any(|(k, _)| k == &lk) {
				self.attrs
					.push((lk, html_escape::decode_html_entities(unquoted).to_string()));
			}
		}
		self.attrs_complete = true;
	}
	fn build_raw_cache(&mut self) {
		let attr_re = Regex::new(
			r#"([a-zA-Z()[\]#@$.?:][a-zA-Z0-9-._:()[\]#]*)(?:\s*=\s*((?:'[^']*')|(?:\"[^\"]*\")|[^\s>]+))?"#,
		)
		.unwrap();
		let mut raw_map = HashMap::new();
		for cap in attr_re.captures_iter(&self.raw_attrs) {
			let key = cap.get(1).unwrap().as_str();
			let value = cap.get(2).map(|m| m.as_str()).unwrap_or("");
			let mut chosen = key.to_string();
			if raw_map.contains_key(&chosen) {
				let mut suffix = 1;
				while raw_map.contains_key(&format!("{}#dup{}", chosen, suffix)) {
					suffix += 1;
				}
				chosen = format!("{}#dup{}", chosen, suffix);
			}
			let mut value = value.trim();
			if (value.starts_with('"') && value.ends_with('"'))
				|| (value.starts_with('\'') && value.ends_with('\''))
			{
				value = &value[1..value.len() - 1];
			}
			raw_map.insert(chosen.clone(), value.to_string());
		}
		self.cache_raw_map = Some(raw_map);
	}

	fn ensure_lower_decoded(&mut self) {
		if self.cache_lower_decoded.is_some() {
			return;
		}
		self.ensure_raw_attributes();
		let mut lower = HashMap::new();
		if let Some(raw) = &self.cache_raw_map {
			for (k, v) in raw {
				lower.insert(
					k.to_lowercase(),
					html_escape::decode_html_entities(v).to_string(),
				);
			}
		}
		self.cache_lower_decoded = Some(lower);
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
