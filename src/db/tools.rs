pub(crate) type TIMESTAMP = Vec<u8>;
pub(crate) type DATETIME = chrono::NaiveDateTime;
pub(crate) type DECIMAL = f64;
pub(crate) type DECIMALL = f64;

pub(crate) fn now() -> DATETIME {
	let now = chrono::Local::now();
	let dt = now.naive_local();
	return dt;
}

pub(crate) fn uuid() -> String {
	return uuid::Uuid::new_v4().to_string();
}
