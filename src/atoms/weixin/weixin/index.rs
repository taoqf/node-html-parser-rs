#[allow(dead_code)]
pub(crate) struct Weixin {
	pub(crate) appid: String,
	pub(crate) appsecret: String,
	get_token_url: String,
	token_key: String,
}

#[allow(dead_code)]
impl Weixin {
	pub(crate) async fn get_access_token(&mut self) -> String {
		let state = crate::get_state().await;
		let token = super::super::token::get_access_token(
			self.appid.as_str(),
			self.appsecret.as_str(),
			self.get_token_url.as_str(),
			&self.token_key,
			state.pg.as_ref(),
		)
		.await;
		return token.token;
	}
}

#[allow(dead_code)]
impl Weixin {
	pub(crate) async fn new() -> Self {
		let appid = std::env::var("WX_APPID").unwrap();
		log::debug!("appid={}", appid);
		let appsecret = std::env::var("WX_APPSECRET").unwrap();
		log::debug!("appsecret={}", appsecret);
		let get_token_url = format!(
			"https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
			appid, appsecret
		);
		let token_key = "wxtoken".to_owned();
		return Self {
			appid: appid.to_owned(),
			appsecret: appsecret.to_owned(),
			get_token_url,
			token_key,
		};
	}
}
