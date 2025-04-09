const URL: &str = "https://qyapi.weixin.qq.com/cgi-bin/user/list";

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Text {
	pub(crate) value: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Web {
	pub(crate) url: String,
	pub(crate) title: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Miniprogram {
	pub(crate) appid: String,
	pub(crate) pagepath: String,
	pub(crate) title: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Attr {
	#[serde(rename = "type")]
	pub(crate) attr_type: i32,
	pub(crate) name: String,
	pub(crate) text: Option<Text>,
	pub(crate) web: Option<Web>,
	pub(crate) miniprogram: Option<Miniprogram>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct ExtAttr {
	pub(crate) attrs: Vec<Attr>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct WechatChannels {
	pub(crate) nickname: String,
	pub(crate) status: isize,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct ExtProfile {
	pub(crate) external_corp_name: String,
	pub(crate) wechat_channels: Option<WechatChannels>,
	pub(crate) external_attr: Vec<Attr>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct User {
	pub(crate) userid: String,
	pub(crate) name: String,
	pub(crate) department: Vec<i32>,
	pub(crate) order: Vec<i32>,
	pub(crate) position: String,
	pub(crate) mobile: Option<String>,
	pub(crate) gender: Option<isize>,
	pub(crate) biz_mail: Option<String>,
	pub(crate) email: Option<String>,
	pub(crate) is_leader_in_dept: Vec<i32>,
	pub(crate) direct_leader: Vec<String>,
	pub(crate) avatar: Option<String>,
	pub(crate) thumb_avatar: Option<String>,
	pub(crate) telephone: String,
	pub(crate) alias: String,
	pub(crate) status: isize,
	pub(crate) address: Option<String>,
	pub(crate) english_name: Option<String>,
	pub(crate) open_userid: Option<String>,
	pub(crate) main_department: i32,
	pub(crate) extattr: ExtAttr,
	pub(crate) qr_code: Option<String>,
	pub(crate) external_position: Option<String>,
	pub(crate) external_profile: Option<ExtProfile>,
	// 以下为文档中没有，实际返回有的
	pub(crate) enable: isize,
	pub(crate) isleader: isize,
	pub(crate) hide_mobile: isize,
}

#[allow(dead_code)]
impl super::index::WeixinWork {
	/// 获取部门成员详情
	/// @see https://developer.work.weixin.qq.com/document/path/90201
	pub(crate) async fn get_dept_users(&self, dept_id: i32) -> Vec<User> {
		#[derive(Debug, serde::Deserialize, serde::Serialize)]
		struct WxApiResult {
			errcode: i32,
			errmsg: String,
			userlist: Vec<User>,
		}
		let token = self.get_access_token().await;
		let url = format!(
			"{}?access_token={}&department_id={}",
			URL,
			token.as_str(),
			dept_id
		);
		let ret = reqwest::get(url).await.unwrap();
		let text = ret.text().await.unwrap();
		log::debug!("msg result: {}", text);
		let ret = serde_json::from_str::<WxApiResult>(&text).unwrap();
		log::debug!("msg result: {:#?}", ret);
		let userlist = ret.userlist;
		return userlist;
	}
}

#[tokio::test]
async fn test_qywx_dept_users() {
	log4rs::init_file("./log4rs.yaml", Default::default()).expect("log4rs config not correct!");
	dotenvy::dotenv().unwrap();
	let state = crate::get_state().await;
	let wx = &state.weixinwork;
	let users = wx.get_dept_users(6).await;
	log::debug!("users: {:#?}", users);
}
