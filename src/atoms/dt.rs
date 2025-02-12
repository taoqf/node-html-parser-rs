use std::str::FromStr;

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

/**
 * 获取当前时间时间戳
 */
#[allow(dead_code)]
pub(crate) fn now_stamp() -> i64 {
	let now = chrono::Utc::now();
	let now = now.timestamp();
	return now;
}

#[allow(dead_code)]
pub(crate) enum DtType {
	DATE,
	TIME,
	DATETIME,
}

/**
 * 将时间戳转换为时间字符串
 */
#[allow(dead_code)]
pub(crate) fn dt2str(dt: &Option<chrono::NaiveDateTime>, dt_type: DtType) -> String {
	match dt {
		None => return "".to_string(),
		Some(dt) => match dt_type {
			DtType::DATE => return dt.format("%Y-%m-%d").to_string(),
			DtType::TIME => return dt.format("%H:%M:%S").to_string(),
			DtType::DATETIME => return dt.format("%Y-%m-%d %H:%M:%S").to_string(),
		},
	}
}

/**
 * 将时间字符串转换为时间戳
 */
#[allow(dead_code)]
pub(crate) fn str2dt(dt_str: &str) -> chrono::NaiveDateTime {
	let dt = chrono::NaiveDateTime::from_str(dt_str);
	return dt.unwrap();
}

/**
 * 将mssql查询出来的日期转换为对应时区日期
 */
#[allow(dead_code)]
pub(crate) fn add_time_zone(dt: &chrono::NaiveDateTime) -> chrono::NaiveDateTime {
	let dt = dt.clone();
	let offset = 8;
	let dt = dt + chrono::Duration::hours(offset);
	return dt;
}

/**
 * 将日期参数转换为数据库查询中使用的UTC时间
 */
#[allow(dead_code)]
pub(crate) fn remove_time_zone(dt: &chrono::NaiveDateTime) -> chrono::NaiveDateTime {
	let dt = dt.clone();
	let offset = 8;
	let dt = dt - chrono::Duration::hours(offset);
	return dt;
}
