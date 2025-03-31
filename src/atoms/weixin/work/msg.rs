const URL: &str = "https://qyapi.weixin.qq.com/cgi-bin/message/send";

#[allow(dead_code)]
impl super::index::WeixinWork {
	/// 发送应用消息
	/// @see https://developer.work.weixin.qq.com/document/path/90236
	pub(crate) async fn msg_send_text<T>(&self, to: &[T], content: T)
	where
		T: AsRef<str>,
	{
		assert!(to.is_empty() == false, "user list could not be empty");
		let content = content.as_ref();
		assert!(content.is_empty() == false, "msg could not be empty");
		let touser = to
			.iter()
			.map(|it| it.as_ref())
			.collect::<Vec<_>>()
			.join("|");
		#[derive(Debug, serde:: Deserialize)]
		struct WxApiResult {
			errcode: i32,
			errmsg: String,
			invaliduser: String,
			invalidparty: String,
			invalidtag: String,
			unlicenseduser: String,
			msgid: String,
			response_code: String,
		}
		let token = self.get_access_token().await;
		let url = format!("{}?access_token={}", URL, token.as_str());
		let param = serde_json::json!({
			"touser": touser,
			"msgtype": "text",
			"agentid": self.agent,
			"content": content,
		});
		log::debug!("msg param:{:#?}", param);
		let client = reqwest::Client::new();
		let ret = client.post(url.as_str()).json(&param).send().await.unwrap();
		let text = ret.text().await.unwrap();
		log::debug!("msg result: {}", text);
		let ret = serde_json::from_str::<WxApiResult>(&text).unwrap();
		log::debug!("msg result: {:#?}", ret);
	}
}

#[allow(dead_code)]
impl super::index::WeixinWork {
	/// 发送markdown消息
	/// @see https://developer.work.weixin.qq.com/document/path/90236
	pub(crate) async fn msg_send_markdown<T>(&self, to: &[T], content: T)
	where
		T: AsRef<str>,
	{
		assert!(to.is_empty() == false, "user list could not be empty");
		let content = content.as_ref();
		assert!(content.is_empty() == false, "msg could not be empty");
		let touser = to
			.iter()
			.map(|it| it.as_ref())
			.collect::<Vec<_>>()
			.join("|");
		#[derive(Debug, serde:: Deserialize)]
		struct WxApiResult {
			errcode: i32,
			errmsg: String,
			invaliduser: String,
			invalidparty: String,
			invalidtag: String,
			unlicenseduser: String,
			msgid: String,
			response_code: String,
		}
		let token = self.get_access_token().await;
		let url = format!("{}?access_token={}", URL, token.as_str());
		let param = serde_json::json!({
			"touser": touser,
			"msgtype": "markdown",
			"agentid": self.agent,
			"markdown": {
				"content": content
			},
		});
		log::debug!("msg param:{:#?}", param);
		let client = reqwest::Client::new();
		let ret = client.post(url.as_str()).json(&param).send().await.unwrap();
		let text = ret.text().await.unwrap();
		log::debug!("msg result: {}", text);
		let ret = serde_json::from_str::<WxApiResult>(&text).unwrap();
		log::debug!("msg result: {:#?}", ret);
	}
}
