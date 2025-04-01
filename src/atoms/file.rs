use super::{md5::md5, uuid::uuid};

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub(crate) struct UploadedFileInfo {
	pub(crate) fileid: String,
	pub(crate) filename: String,
	pub(crate) filetype: String,
}

#[allow(dead_code)]
pub(crate) async fn upload(file_content: bytes::Bytes, file_name: &str) -> UploadedFileInfo {
	let client = reqwest::Client::new();
	let state = crate::get_state().await;
	let url = format!("{}/upload", &state.file_server);
	let request = client.put(url);
	let request = if state.file_msg_encode_enable {
		let random = super::dt::now_stamp();
		let safe_code = &state.file_msg_encode_safe_key.clone();
		let annonce = uuid();
		let token = md5(format!("{safe_code}{random}{annonce}{random}{safe_code}"));
		// let cookie = format!(
		// 	"x-appid={};x-token={};x-random={};x-annonce={}",
		// 	&state.appid, token, random, annonce
		// );
		// let cookie = reqwest::header::HeaderValue::from_str(&cookie).unwrap();
		request
			.header(
				"x-appid",
				reqwest::header::HeaderValue::from_str(&state.appid).unwrap(),
			)
			.header(
				"x-token",
				reqwest::header::HeaderValue::from_str(&token).unwrap(),
			)
			.header(
				"x-random",
				reqwest::header::HeaderValue::from_str(&random.to_string()).unwrap(),
			)
			.header(
				"x-annonce",
				reqwest::header::HeaderValue::from_str(&annonce).unwrap(),
			)
	} else {
		request
	};

	let file = reqwest::multipart::Part::bytes(file_content.to_vec())
		.file_name(file_name.to_owned())
		// .content_type("application/octet-stream")
		.mime_str("application/octet-stream")
		.unwrap();

	let form = reqwest::multipart::Form::new().part("file", file);

	let response = request.multipart(form).send().await.unwrap();

	let str = response.text().await.unwrap();
	log::debug!("file uploaded: {}", str);
	let file_info = serde_json::from_str::<UploadedFileInfo>(&str).unwrap();
	return file_info;
}

#[allow(dead_code)]
pub(crate) struct DownloadFileInfo {
	pub(crate) filename: String,
	pub(crate) filecontent: bytes::Bytes,
}

#[allow(dead_code)]
pub(crate) async fn download(url: &str) -> DownloadFileInfo {
	let client = reqwest::Client::new();
	let ret = client.get(url).send().await.unwrap();
	let headers = ret.headers();
	let filename = get_file_name_from_disposition(headers);
	let bytes = ret.bytes().await.unwrap();
	return DownloadFileInfo {
		filename,
		filecontent: bytes,
	};
}

#[allow(dead_code)]
fn get_file_name_from_disposition(headers: &reqwest::header::HeaderMap) -> String {
	let re = regex::bytes::Regex::new(r"filename\*?=[^;]*''?([^;]*)").unwrap();
	let disposition = headers.get("Content-Disposition");
	if let Some(disposition) = disposition {
		let disposition = disposition.to_str().unwrap_or_default();
		if let Some(caps) = re.captures(disposition.as_bytes()) {
			if let Some(file_name) = caps.get(1) {
				let file_name = std::str::from_utf8(file_name.as_bytes()).unwrap();
				let file_name = file_name.trim_matches('"');
				// 解码文件名
				if let Ok(decoded_name) =
					percent_encoding::percent_decode_str(file_name).decode_utf8()
				{
					return decoded_name.to_string();
				}
			}
		}
	}
	return super::uuid::uuid();
}
