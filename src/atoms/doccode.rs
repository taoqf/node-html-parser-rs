use chrono::{Datelike, Timelike};

const URI: &str = "http://doccode:8890";

#[allow(dead_code)]
pub(crate) async fn doccode(name: &str, num: usize, len: usize) -> Vec<String> {
	if len == 0 {
		return vec![];
	}
	let client = reqwest::Client::new();
	let data = client
		.post(URI)
		.json(&serde_json::json!({
		"name": name,
		"num": num,
		"len": len,
			}))
		.send()
		.await
		.unwrap()
		.json::<Vec<String>>()
		.await
		.unwrap();
	return data;
}

#[allow(dead_code)]
pub(crate) async fn doccode_with_dt(name: &str, num: usize, len: usize) -> Vec<String> {
	if len == 0 {
		return vec![];
	}
	let now = crate::atoms::dt::now();
	let year = now.year();
	let month = now.month();
	let day = now.day();
	let name = format!("{name}{:04}{:02}{:02}", year, month, day);

	return doccode(&name, num, len).await;
}

#[allow(dead_code)]
pub(crate) async fn doccode_with_tm(name: &str, num: usize, len: usize) -> Vec<String> {
	if len == 0 {
		return vec![];
	}
	let now = crate::atoms::dt::now();
	let year = now.year();
	let month = now.month();
	let day = now.day();
	let hour = now.hour();
	let minute = now.minute();
	let second = now.second();
	let name = format!(
		"{name}{:04}{:02}{:02}{:02}{:02}{:02}",
		year, month, day, hour, minute, second
	);

	return doccode(&name, num, len).await;
}
