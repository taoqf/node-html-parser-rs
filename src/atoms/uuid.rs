#[allow(dead_code)]
pub(crate) fn uuid() -> String {
	return uuid::Uuid::new_v4().to_string();
}
