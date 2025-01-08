pub(crate) fn now() -> DATETIME {
	let now = chrono::Local::now();
	let dt = now.naive_local();
	return dt;
}
