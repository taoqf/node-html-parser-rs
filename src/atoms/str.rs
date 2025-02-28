#[allow(dead_code)]
pub(crate) fn is_none_or_empty(s: Option<&str>) -> bool {
	match s {
		Some(s) => s.is_empty(),
		None => true,
	}
}

#[allow(dead_code)]
pub(crate) fn get_opt_str(val: &Option<String>) -> &str {
	match val {
		Some(val) => val.as_str(),
		None => "",
	}
}
