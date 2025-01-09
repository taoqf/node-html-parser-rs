#[derive(Debug, serde::Deserialize)]
pub(crate) struct Code2sessionResult {
	/** 用户唯一标识 */
	openid: String,
	/** 会话密钥 */
	session_key: String,
	/** 用户在开放平台的唯一标识符，若当前小程序已绑定到微信开放平台帐号或绑定到微信开放平台帐号的小程序已开通微信支付，则返回 */
	unionid: String,
}

pub(crate) async fn code2session(
	code: &str,
) -> Result<Code2sessionResult, Box<dyn std::error::Error>> {
	log4rs::init_file("./log4rs.yaml", Default::default()).expect("log4rs config not correct!");
	dotenvy::dotenv().unwrap();
	let appid = std::env::var("WX_APPID").unwrap();
	let secret = std::env::var("WX_APPSECRET").unwrap();
	let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code");
	// !!! 目前rust无法继承另外一个结构体
	#[derive(Debug, serde::Deserialize)]
	struct Result {
		errcode: i32,
		errmsg: String,
		/** 用户唯一标识 */
		openid: String,
		/** 会话密钥 */
		session_key: String,
		/** 用户在开放平台的唯一标识符，若当前小程序已绑定到微信开放平台帐号或绑定到微信开放平台帐号的小程序已开通微信支付，则返回 */
		unionid: String,
	}
	let ret = reqwest::get(url).await?.json::<Result>().await?;
	log::debug!("code2session: {:?}", ret);
	let Result {
		errcode,
		errmsg,
		openid,
		session_key,
		unionid,
	} = ret;
	if errcode != 0 {
		return Err(errmsg.into());
	} else {
		return Ok(Code2sessionResult {
			openid,
			session_key,
			unionid,
		});
	}
}
