#[allow(dead_code)]
pub(crate) fn is_none_or_empty(s: Option<&str>) -> bool {
	match s {
		Some(s) => s.is_empty(),
		None => true,
	}
}

#[allow(dead_code)]
pub(crate) fn opt2str(val: &Option<String>) -> &str {
	match val {
		Some(val) => val.as_str(),
		None => "",
	}
}

#[allow(dead_code)]
pub(crate) fn str2opt(val: &str) -> Option<String> {
	return Some(val.to_owned());
}

#[allow(dead_code)]
pub(crate) fn sha1(val: &str) -> String {
	use crypto::digest::Digest;
	let mut hasher = crypto::sha1::Sha1::new();
	hasher.input_str(val);
	let hex = hasher.result_str();
	return hex;
}

// #[allow(dead_code)]
// pub(crate) fn md51<T: AsRef<[u8]>>(input: T) -> String {
// 	let str = md5::compute(input);
// 	let hash = format!("{:x}", str);
// 	return hash;
// }

#[allow(dead_code)]
pub(crate) fn md5(input: &str) -> String {
	use crypto::digest::Digest;
	let mut hasher = crypto::md5::Md5::new();
	hasher.input_str(input);
	let hex = hasher.result_str();
	return hex;
}

#[allow(dead_code)]
pub(crate) fn merge_json(a: &mut serde_json::Value, b: &serde_json::Value) {
	if let (serde_json::Value::Object(a_map), serde_json::Value::Object(b_map)) = (a, b) {
		for (key, value) in b_map {
			a_map.entry(key).or_insert(value.clone());
		}
	}
}

#[allow(dead_code)]
pub(crate) fn uuid() -> String {
	return uuid::Uuid::new_v4().to_string();
}
