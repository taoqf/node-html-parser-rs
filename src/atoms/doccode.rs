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
