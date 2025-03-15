#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Sheet {
	pub(crate) docid: String,
	pub(crate) sheet_id: String,
	pub(crate) title: String,
	pub(crate) index: i32,
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 添加子表
	/// @see https://developer.work.weixin.qq.com/document/path/100214
	pub(crate) async fn doc_smartsheet_add(&mut self, docid: &str, sheet_name: &str) -> Sheet {
		assert!(docid.is_empty() == false, "doc_id could not be empty");
		assert!(
			sheet_name.is_empty() == false,
			"sheet_name could not be empty"
		);
		assert!(
			sheet_name.len() <= 255,
			"docname could not be more than 255 characters"
		);
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/smartsheet/add_sheet?access_token={token}"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct Property {
			sheet_id: String,
			title: String,
			index: i32,
		}
		#[derive(Debug, serde:: Deserialize)]
		struct AddSheetResult {
			errcode: i32,
			errmsg: String,
			properties: Property,
		}
		let client = reqwest::Client::new();
		let ret = client
			.post(url)
			.json(&serde_json::json!({
				"docid":docid,
				"properties": {
					"title": sheet_name,
					// "index": 3,
					// "sheet_id": "123abc"
				}
			}))
			.send()
			.await
			.unwrap()
			.json::<AddSheetResult>()
			.await
			.unwrap();
		log::debug!("add sheet record result: {:?}", ret);
		assert!(ret.errcode == 0, "failed to add sheet: {}", ret.errmsg);
		let sheet = ret.properties;
		return Sheet {
			docid: docid.to_owned(),
			sheet_id: sheet.sheet_id,
			title: sheet.title,
			index: sheet.index,
		};
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 添加记录
	/// @see https://developer.work.weixin.qq.com/document/path/100224
	/// records 参见 https://developer.work.weixin.qq.com/document/path/100224#addrecord，
	/// 由于其灵活性，不方便封装为结构体，可在使用时根据官方文档生成。
	pub(crate) async fn doc_smartsheet_record_add(
		&mut self,
		docid: &str,
		sheet_id: &str,
		records: Vec<serde_json::Value>,
	) {
		assert!(docid.is_empty() == false, "doc_id could not be empty");
		assert!(sheet_id.is_empty() == false, "sheet_id could not be empty");
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/smartsheet/add_records?access_token={token}"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct AddSheetRecordResult {
			errcode: i32,
			errmsg: String,
			records: serde_json::Value,
		}
		let client = reqwest::Client::new();
		let ret = client
			.post(url)
			.json(&serde_json::json!({
				"docid":docid,
				"sheet_id":sheet_id,
				"key_type": "CELL_VALUE_KEY_TYPE_FIELD_TITLE",
				"records": records
			}))
			.send()
			.await
			.unwrap()
			.json::<AddSheetRecordResult>()
			.await
			.unwrap();
		log::debug!("add records result: {:?}", ret);
		assert!(ret.errcode == 0, "failed to add records: {}", ret.errmsg);
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 查询记录
	/// @see https://developer.work.weixin.qq.com/document/path/100230
	pub(crate) async fn doc_smartsheet_record_get(&mut self, docid: &str, sheet_id: &str) {
		assert!(docid.is_empty() == false, "doc_id could not be empty");
		assert!(sheet_id.is_empty() == false, "sheet_id could not be empty");
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/smartsheet/get_records?access_token={token}"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct GetSheetRecordResult {
			errcode: i32,
			errmsg: String,
			records: Vec<serde_json::Value>,
			total: u32,
			next: u32,
			has_more: bool,
		}
		let client = reqwest::Client::new();
		let mut offset = 0;
		let mut records: Vec<serde_json::Value> = Vec::new();
		loop {
			let ret = client
				.post(&url)
				.json(&serde_json::json!({
					"docid": docid,
					"sheet_id": sheet_id,
					"key_type": "CELL_VALUE_KEY_TYPE_FIELD_TITLE",
					"offset": offset,
					"limit": 1000	// 最大1000
				}))
				.send()
				.await
				.unwrap()
				.json::<GetSheetRecordResult>()
				.await
				.unwrap();
			log::debug!("get records result: {:?}", ret);
			assert!(ret.errcode == 0, "failed to get records: {}", ret.errmsg);
			offset = ret.next;
			records.extend(ret.records);
			if ret.has_more == false {
				break;
			}
		}
		return;
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 更新记录
	/// @see https://developer.work.weixin.qq.com/document/path/100226
	/// records 参见 https://developer.work.weixin.qq.com/document/path/100226#updaterecord，
	/// 由于其灵活性，不方便封装为结构体，可在使用时根据官方文档生成。
	pub(crate) async fn doc_smartsheet_record_update(
		&mut self,
		docid: &str,
		sheet_id: &str,
		records: Vec<serde_json::Value>,
	) {
		assert!(docid.is_empty() == false, "doc_id could not be empty");
		assert!(sheet_id.is_empty() == false, "sheet_id could not be empty");
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/smartsheet/update_records?access_token={token}"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct AddSheetRecordResult {
			errcode: i32,
			errmsg: String,
			records: serde_json::Value,
		}
		let client = reqwest::Client::new();
		let ret = client
			.post(url)
			.json(&serde_json::json!({
				"docid":docid,
				"sheet_id":sheet_id,
				"key_type": "CELL_VALUE_KEY_TYPE_FIELD_TITLE",
				"records": records
			}))
			.send()
			.await
			.unwrap()
			.json::<AddSheetRecordResult>()
			.await
			.unwrap();
		log::debug!("update records result: {:?}", ret);
		assert!(ret.errcode == 0, "failed to update records: {}", ret.errmsg);
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 删除记录
	/// @see https://developer.work.weixin.qq.com/document/path/100225
	pub(crate) async fn doc_smartsheet_record_del(
		&mut self,
		docid: &str,
		sheet_id: &str,
		records: Vec<String>,
	) {
		assert!(docid.is_empty() == false, "doc_id could not be empty");
		assert!(sheet_id.is_empty() == false, "sheet_id could not be empty");
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/smartsheet/delete_records?access_token={token}"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct DelSheetRecordResult {
			errcode: i32,
			errmsg: String,
		}
		let client = reqwest::Client::new();
		let ret = client
			.post(url)
			.json(&serde_json::json!({
				"docid":docid,
				"sheet_id":sheet_id,
				"key_type": "CELL_VALUE_KEY_TYPE_FIELD_TITLE",
				"records": records
			}))
			.send()
			.await
			.unwrap()
			.json::<DelSheetRecordResult>()
			.await
			.unwrap();
		log::debug!("del sheet records result: {:?}", ret);
		assert!(ret.errcode == 0, "failed to delete records: {}", ret.errmsg);
	}
}
