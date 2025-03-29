#[allow(dead_code)]
pub(crate) enum DocType {
	/// 文档
	Doc = 3,
	/// 表格
	Sheet = 4,
	/// 智能表格
	SmartSheet = 10,
}

// impl Into<i32> for DocType {
// 	fn into(self) -> i32 {
// 		match self {
// 			DocType::Doc => 3,
// 			DocType::Sheet => 4,
// 			DocType::SmartSheet => 10,
// 		}
// 	}
// }

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Doc {
	pub(crate) url: String,
	pub(crate) docid: String,
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 新建文档
	/// @see https://developer.work.weixin.qq.com/document/path/97470
	pub(crate) async fn doc_create(
		&self,
		spaceid: &str,
		doc_type: DocType,
		doc_name: &str,
		admin_users: &[&str],
	) -> Doc {
		assert!(doc_name.is_empty() == false, "docname could not be empty");
		assert!(
			doc_name.len() <= 255,
			"docname could not be more than 255 characters"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct CreateDocResult {
			errcode: i32,
			errmsg: String,
			url: String,
			docid: String,
		}
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/create_doc?access_token={}",
			token.as_str()
		);
		let param = if spaceid.is_empty() {
			serde_json::json!({
				"doc_type": doc_type as i32,
				"doc_name": doc_name,
				"admin_users": admin_users,
			})
		} else {
			serde_json::json!({
				"spaceid": spaceid,
				"fatherid": spaceid,
				"doc_type": doc_type as i32,
				"doc_name": doc_name,
				"admin_users": admin_users,
			})
		};
		log::debug!("create doc param:{:#?}", param);
		let client = reqwest::Client::new();
		let ret = client.post(url.as_str()).json(&param).send().await.unwrap();
		let text = ret.text().await.unwrap();
		log::debug!("create doc result: {}", text);
		let ret = serde_json::from_str::<CreateDocResult>(&text).unwrap();
		// let ret = client
		// 	.post(url)
		// 	.json(&serde_json::json!({
		// 		"doc_type":doc_type as i32,
		// 		"doc_name":doc_name,
		// 		"admin_users":admin_users,
		// 	}))
		// 	.send()
		// 	.await
		// 	.unwrap()
		// 	.json::<CreateDocResult>()
		// 	.await
		// 	.unwrap();
		// log::debug!("create doc result: {:?}", ret);
		assert!(ret.errcode == 0, "create doc failed: {}", ret.errmsg);
		return Doc {
			docid: ret.docid,
			url: ret.url,
		};
	}
}

#[allow(dead_code)]
pub(crate) struct Sheet {
	pub(crate) docid: String,
	pub(crate) sheet_id: String,
	pub(crate) title: String,
	pub(crate) is_visible: bool,
	pub(crate) sheet_type: String, //"smartsheet"
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 查询
	/// @see https://developer.work.weixin.qq.com/document/path/99911
	pub(crate) async fn doc_get_sheets(&self, docid: &str) -> Vec<Sheet> {
		assert!(docid.is_empty() == false, "doc_id could not be empty");
		assert!(
			docid.len() <= 255,
			"docname could not be more than 255 characters"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct SheetData {
			sheet_id: String,
			title: String,
			is_visible: bool,
			#[serde(rename = "type")]
			sheet_type: String, //"smartsheet"
		}
		#[derive(Debug, serde:: Deserialize)]
		struct GetSheetsResult {
			errcode: i32,
			errmsg: String,
			sheet_list: Vec<SheetData>,
		}
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/smartsheet/get_sheet?access_token={}",
			token.as_str()
		);
		let param = serde_json::json!({
			"docid":docid,
		});
		log::debug!("get doc sheets param:{:#?}", param);
		let client = reqwest::Client::new();
		let ret = client.post(url.as_str()).json(&param).send().await.unwrap();
		let text = ret.text().await.unwrap();
		log::debug!("get doc sheets result: {}", text);
		let ret = serde_json::from_str::<GetSheetsResult>(&text).unwrap();
		// let ret = client
		// 	.post(url)
		// 	.json(&serde_json::json!({
		// 		"doc_type":doc_type as i32,
		// 		"doc_name":doc_name,
		// 		"admin_users":admin_users,
		// 	}))
		// 	.send()
		// 	.await
		// 	.unwrap()
		// 	.json::<CreateDocResult>()
		// 	.await
		// 	.unwrap();
		// log::debug!("create doc result: {:?}", ret);
		assert!(ret.errcode == 0, "get doc sheets failed: {}", ret.errmsg);
		return ret
			.sheet_list
			.iter()
			.map(|sheet| Sheet {
				docid: docid.to_owned(),
				is_visible: sheet.is_visible,
				sheet_id: sheet.sheet_id.clone(),
				sheet_type: sheet.sheet_type.clone(),
				title: sheet.title.clone(),
			})
			.collect();
	}
}
