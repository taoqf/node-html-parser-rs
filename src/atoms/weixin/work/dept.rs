const URL: &str = "https://qyapi.weixin.qq.com/cgi-bin/department/list";

#[allow(dead_code)]
#[derive(Debug, serde:: Deserialize)]
pub(crate) struct Department {
	pub(crate) id: i32,
	pub(crate) name: String,
	pub(crate) name_en: Option<String>,
	pub(crate) department_leader: Vec<String>,
	pub(crate) parentid: Option<i32>,
	pub(crate) order: i32,
}

#[allow(dead_code)]
impl super::index::WeixinWork {
	/// 获取部门列表
	/// @see https://developer.work.weixin.qq.com/document/path/90208
	pub(crate) async fn get_depts(&self) -> Vec<Department> {
		#[derive(Debug, serde:: Deserialize)]
		struct WxApiResult {
			errcode: i32,
			errmsg: String,
			department: Vec<Department>,
		}
		let token = self.get_access_token().await;
		let url = format!("{}?access_token={}", URL, token.as_str());
		let ret = reqwest::get(url).await.unwrap();
		let text = ret.text().await.unwrap();
		log::debug!("msg result: {}", text);
		let ret = serde_json::from_str::<WxApiResult>(&text).unwrap();
		log::debug!("msg result: {:#?}", ret);
		let department = ret.department;
		return department;
	}
}

#[tokio::test]
async fn test_qywx_depts() {
	log4rs::init_file("./log4rs.yaml", Default::default()).expect("log4rs config not correct!");
	dotenvy::dotenv().unwrap();
	let state = crate::get_state().await;
	let wx = &state.weixinwork;
	let depts = wx.get_depts().await;
	log::debug!("departments: {:#?}", depts);
}
