#[allow(dead_code)]
impl super::index::WeixinWork {
	/// 下载文件
	/// @see https://developer.work.weixin.qq.com/document/path/97881
	pub(crate) async fn wedrive_download(&self, fileid: &str) -> bytes::Bytes {
		assert!(fileid.is_empty() == false, "fileid could not be empty");
		#[derive(Debug, serde:: Deserialize)]
		struct WxApiResult {
			errcode: i32,
			errmsg: String,
			download_url: String,
			cookie_name: String,
			cookie_value: String,
		}
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedrive/file_download?access_token={}",
			token.as_str()
		);
		let param = serde_json::json!({
			"fileid": fileid,
		});
		log::debug!("download file param:{:#?}", param);
		let client = reqwest::Client::new();
		let ret = client.post(url.as_str()).json(&param).send().await.unwrap();
		let text = ret.text().await.unwrap();
		log::debug!("download file result: {}", text);
		let ret = serde_json::from_str::<WxApiResult>(&text).unwrap();
		let cookie = format!("{}={}", ret.cookie_name, ret.cookie_value);
		let cookie = reqwest::header::HeaderValue::from_str(&cookie).unwrap();
		let ret = client
			.get(ret.download_url)
			.header(reqwest::header::COOKIE, cookie)
			.send()
			.await
			.unwrap();
		let bytes = ret.bytes().await.unwrap();
		return bytes;
	}
}

#[allow(dead_code)]
impl super::index::WeixinWork {
	/// 删除文件
	/// @see https://developer.work.weixin.qq.com/document/path/97885
	pub(crate) async fn wedrive_delete(&self, fileid: &[&str]) -> bool {
		assert!(fileid.is_empty() == false, "fileid could not be empty");
		#[derive(Debug, serde:: Deserialize)]
		struct WxApiResult {
			errcode: i32,
			errmsg: String,
		}
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedrive/file_delete?access_token={}",
			token.as_str()
		);
		let param = serde_json::json!({
			"fileid": fileid,
		});
		log::debug!("delete file param:{:#?}", param);
		let client = reqwest::Client::new();
		let ret = client.post(url.as_str()).json(&param).send().await.unwrap();
		let text = ret.text().await.unwrap();
		log::debug!("delete file result: {}", text);
		let ret = serde_json::from_str::<WxApiResult>(&text).unwrap();
		return ret.errcode == 0;
	}
}

#[allow(dead_code)]
impl super::index::WeixinWork {
	/// 新建空间
	/// @see https://developer.work.weixin.qq.com/document/path/96845
	pub(crate) async fn wedrive_create_space(&self, space_name: &str) -> String {
		assert!(
			space_name.is_empty() == false,
			"space_name could not be empty"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct WxApiResult {
			errcode: i32,
			errmsg: String,
			spaceid: String,
		}
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedrive/space_create?access_token={}",
			token.as_str()
		);
		let param = serde_json::json!({
			"space_name": space_name,
		});
		log::debug!("delete file param:{:#?}", param);
		let client = reqwest::Client::new();
		let ret = client.post(url.as_str()).json(&param).send().await.unwrap();
		let text = ret.text().await.unwrap();
		log::debug!("delete file result: {}", text);
		let ret = serde_json::from_str::<WxApiResult>(&text).unwrap();
		return ret.spaceid;
	}
}
