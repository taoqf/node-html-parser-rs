pub(crate) fn is_none_or_empty(s: Option<&str>) -> bool {
	match s {
		Some(s) => s.is_empty(),
		None => true,
	}
}
