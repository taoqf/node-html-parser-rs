use super::types::DATETIME;

/**
 * 获取当前时间
 */
#[allow(dead_code)]
pub(crate) fn now() -> DATETIME {
	let now = chrono::Local::now();
	let dt = now.naive_local();
	return dt;
}
