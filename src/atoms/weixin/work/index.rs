#[allow(dead_code)]
pub(crate) struct WeixinWork {
	pub(super) appid: String,
	pub(super) appsecret: String,
	pub(super) agent: u32,
	pub(super) db: Box<dyn welds::Client>,
	pub(super) get_token_url: String,
	pub(super) token_key: String,
}

#[allow(dead_code)]
impl WeixinWork {
	pub(crate) async fn get_access_token(&self) -> String {
		let token = super::super::token::get_access_token(
			self.appid.as_str(),
			self.appsecret.as_str(),
			self.get_token_url.as_str(),
			&self.token_key,
			self.db.as_ref(),
		)
		.await;
		log::debug!("token={:#?}", token);
		return token.token;
	}
}

#[allow(dead_code)]
impl WeixinWork {
	pub(crate) async fn new() -> Self {
		let appid = std::env::var("QYWX_APPID").unwrap();
		log::debug!("appid={}", appid);
		let appsecret = std::env::var("QYWX_APPSECRET").unwrap();
		log::debug!("appsecret={}", appsecret);
		let agent = std::env::var("QYWX_AGENTID").unwrap().parse().unwrap();
		log::debug!("agent={}", agent);
		let db = crate::atoms::db::get_db("DB_PG").await;
		let get_token_url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}",
			appid, appsecret
		);
		let token_key = format!("qywxtoken{}", agent);
		return Self {
			appid: appid.to_owned(),
			appsecret: appsecret.to_owned(),
			agent,
			db,
			get_token_url,
			token_key,
		};
	}
}
