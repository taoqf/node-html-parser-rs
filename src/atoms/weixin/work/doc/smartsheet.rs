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
	pub(crate) async fn doc_smartsheet_add(&self, docid: &str, sheet_name: &str) -> Sheet {
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
			.unwrap();
		let ret = ret.text().await.unwrap();
		log::debug!("add sheet result: {:?}", ret);
		let ret = serde_json::from_str::<AddSheetResult>(&ret).unwrap();
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
		&self,
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
			.unwrap();
		let ret = ret.text().await.unwrap();
		log::debug!("add sheet record result: {:?}", ret);
		let ret = serde_json::from_str::<AddSheetRecordResult>(&ret).unwrap();
		log::debug!("add records result: {:?}", ret);
		assert!(ret.errcode == 0, "failed to add records: {}", ret.errmsg);
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 查询记录
	/// @see https://developer.work.weixin.qq.com/document/path/100230
	pub(crate) async fn doc_smartsheet_record_get(&self, docid: &str, sheet_id: &str) {
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
				.unwrap();
			let ret = ret.text().await.unwrap();
			let ret = serde_json::from_str::<GetSheetRecordResult>(&ret).unwrap();
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
		&self,
		docid: &str,
		sheet_id: &str,
		records: &Vec<serde_json::Value>,
	) {
		assert!(docid.is_empty() == false, "doc_id could not be empty");
		assert!(sheet_id.is_empty() == false, "sheet_id could not be empty");
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/smartsheet/update_records?access_token={token}"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct UpdateSheetRecordResult {
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
			.unwrap();
		let ret = ret.text().await.unwrap();
		let ret = serde_json::from_str::<UpdateSheetRecordResult>(&ret).unwrap();
		log::debug!("update records result: {:?}", ret);
		assert!(ret.errcode == 0, "failed to update records: {}", ret.errmsg);
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 删除记录
	/// @see https://developer.work.weixin.qq.com/document/path/100225
	pub(crate) async fn doc_smartsheet_record_del(
		&self,
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
			.unwrap();
		let ret = ret.text().await.unwrap();
		let ret = serde_json::from_str::<DelSheetRecordResult>(&ret).unwrap();
		log::debug!("del sheet records result: {:?}", ret);
		assert!(ret.errcode == 0, "failed to delete records: {}", ret.errmsg);
	}
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct NumberFieldProperty {
	pub(crate) decimal_places: i32,
	pub(crate) use_separate: bool,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct CheckboxFieldProperty {
	pub(crate) checked: bool,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct DateTimeFieldProperty {
	pub(crate) format: String,
	pub(crate) auto_fill: bool,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct AttachmentFieldProperty {
	pub(crate) display_mode: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct UserFieldProperty {
	pub(crate) is_multiple: bool,
	pub(crate) is_notified: bool,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct UrlFieldProperty {
	#[serde(rename = "type")]
	pub(crate) url_type: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct SelectOption {
	pub(crate) id: String,
	pub(crate) text: String,
	pub(crate) style: i32,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct SelectFieldProperty {
	pub(crate) is_quick_add: bool,
	pub(crate) options: Vec<SelectOption>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct CreatedTimeFieldProperty {
	pub(crate) format: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct ModifiedTimeFieldProperty {
	pub(crate) format: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct ProgressFieldProperty {
	pub(crate) decimal_places: i32,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct SingleSelectFieldProperty {
	pub(crate) is_quick_add: bool,
	pub(crate) options: Vec<SelectOption>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct ReferenceFieldProperty {
	/// 关联的子表id，为空时，表示关联本子表
	pub(crate) sub_id: Option<String>,
	pub(crate) filed_id: String,
	pub(crate) is_multiple: bool,
	pub(crate) view_id: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct LocationFieldProperty {
	pub(crate) input_type: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct NumberRule {
	#[serde(rename = "type")]
	pub(crate) rule_type: String,
	pub(crate) value: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct AutoNumberFieldProperty {
	#[serde(rename = "type")]
	pub(crate) auto_type: String,
	pub(crate) rules: Vec<NumberRule>,
	pub(crate) reformat_existing_record: bool,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct CurrencyFieldProperty {
	pub(crate) currency_type: String,
	pub(crate) decimal_places: i32,
	pub(crate) use_separate: bool,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct WwGroupFieldProperty {
	pub(crate) allow_multiple: bool,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize)]
pub(crate) struct Field {
	pub(crate) field_id: String,
	pub(crate) field_title: String,
	pub(crate) field_type: String,
	pub(crate) property_number: Option<NumberFieldProperty>,
	pub(crate) property_checkbox: Option<CheckboxFieldProperty>,
	pub(crate) property_date_time: Option<DateTimeFieldProperty>,
	pub(crate) property_attachment: Option<AttachmentFieldProperty>,
	pub(crate) property_user: Option<UserFieldProperty>,
	pub(crate) property_url: Option<UrlFieldProperty>,
	pub(crate) property_select: Option<SelectFieldProperty>,
	pub(crate) property_created_time: Option<CreatedTimeFieldProperty>,
	pub(crate) property_modified_time: Option<ModifiedTimeFieldProperty>,
	pub(crate) property_progress: Option<ProgressFieldProperty>,
	pub(crate) property_single_select: Option<SingleSelectFieldProperty>,
	pub(crate) property_reference: Option<ReferenceFieldProperty>,
	pub(crate) property_location: Option<LocationFieldProperty>,
	pub(crate) property_auto_number: Option<AutoNumberFieldProperty>,
	pub(crate) property_currency: Option<CurrencyFieldProperty>,
	pub(crate) property_ww_group: Option<WwGroupFieldProperty>,
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 查询字段
	/// @see https://developer.work.weixin.qq.com/document/path/99914
	pub(crate) async fn doc_smartsheet_fields_get(
		&self,
		docid: &str,
		sheet_id: &str,
	) -> Vec<Field> {
		assert!(docid.is_empty() == false, "doc_id could not be empty");
		assert!(sheet_id.is_empty() == false, "sheet_id could not be empty");
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/smartsheet/get_fields?access_token={token}"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct SheetGetFieldsResult {
			errcode: i32,
			errmsg: String,
			total: i32,
			fields: Vec<Field>,
		}
		let client = reqwest::Client::new();
		let ret = client
			.post(url)
			.json(&serde_json::json!({
				"docid":docid,
				"sheet_id":sheet_id,
			}))
			.send()
			.await
			.unwrap();
		let ret = ret.text().await.unwrap();
		let ret = serde_json::from_str::<SheetGetFieldsResult>(&ret).unwrap();
		log::debug!("get sheet fields result: {:?}", ret);
		assert!(
			ret.errcode == 0,
			"failed to get sheet fields: {}",
			ret.errmsg
		);
		return ret.fields;
	}
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde:: Deserialize, Default)]
pub(crate) struct AddField {
	// pub(crate) field_id: String, // 自动生成
	pub(crate) field_title: String,
	pub(crate) field_type: String,
	pub(crate) property_number: Option<NumberFieldProperty>,
	pub(crate) property_checkbox: Option<CheckboxFieldProperty>,
	pub(crate) property_date_time: Option<DateTimeFieldProperty>,
	pub(crate) property_attachment: Option<AttachmentFieldProperty>,
	pub(crate) property_user: Option<UserFieldProperty>,
	pub(crate) property_url: Option<UrlFieldProperty>,
	pub(crate) property_select: Option<SelectFieldProperty>,
	pub(crate) property_created_time: Option<CreatedTimeFieldProperty>,
	pub(crate) property_modified_time: Option<ModifiedTimeFieldProperty>,
	pub(crate) property_progress: Option<ProgressFieldProperty>,
	pub(crate) property_single_select: Option<SingleSelectFieldProperty>,
	pub(crate) property_reference: Option<ReferenceFieldProperty>,
	pub(crate) property_location: Option<LocationFieldProperty>,
	pub(crate) property_auto_number: Option<AutoNumberFieldProperty>,
	pub(crate) property_currency: Option<CurrencyFieldProperty>,
	pub(crate) property_ww_group: Option<WwGroupFieldProperty>,
}

#[allow(dead_code)]
impl AddField {
	pub(crate) fn new(field_title: &str, field_type: &str) -> Self {
		Self {
			field_title: field_title.to_string(),
			field_type: field_type.to_string(),
			..Default::default()
		}
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 添加字段
	/// @see https://developer.work.weixin.qq.com/document/path/99904
	pub(crate) async fn doc_smartsheet_fields_add(
		&self,
		docid: &str,
		sheet_id: &str,
		fields: &Vec<AddField>,
	) {
		assert!(docid.is_empty() == false, "doc_id could not be empty");
		assert!(sheet_id.is_empty() == false, "sheet_id could not be empty");
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/smartsheet/add_fields?access_token={token}"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct AddFieldsResult {
			errcode: i32,
			errmsg: String,
			fields: Vec<Field>,
		}
		let client = reqwest::Client::new();
		let ret = client
			.post(url)
			.json(&serde_json::json!({
				"docid":docid,
				"sheet_id":sheet_id,
				"fields":fields,
			}))
			.send()
			.await
			.unwrap();
		let ret = ret.text().await.unwrap();
		let ret = serde_json::from_str::<AddFieldsResult>(&ret).unwrap();
		log::debug!("add sheet fields result: {:?}", ret);
		assert!(
			ret.errcode == 0,
			"failed to add sheet fields: {}",
			ret.errmsg
		);
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 更新字段
	/// @see https://developer.work.weixin.qq.com/document/path/99906
	pub(crate) async fn doc_smartsheet_fields_update(
		&self,
		docid: &str,
		sheet_id: &str,
		fields: &Vec<Field>,
	) {
		assert!(docid.is_empty() == false, "doc_id could not be empty");
		assert!(sheet_id.is_empty() == false, "sheet_id could not be empty");
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/smartsheet/update_fields?access_token={token}"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct UpdateFieldsResult {
			errcode: i32,
			errmsg: String,
			fields: Vec<Field>,
		}
		let client = reqwest::Client::new();
		let ret = client
			.post(url)
			.json(&serde_json::json!({
				"docid":docid,
				"sheet_id":sheet_id,
				"fields":fields,
			}))
			.send()
			.await
			.unwrap();
		let ret = ret.text().await.unwrap();
		let ret = serde_json::from_str::<UpdateFieldsResult>(&ret).unwrap();
		log::debug!("update sheet fields result: {:?}", ret);
		assert!(
			ret.errcode == 0,
			"failed to update sheet fields: {}",
			ret.errmsg
		);
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 删除字段
	/// @see https://developer.work.weixin.qq.com/document/path/99905
	pub(crate) async fn doc_smartsheet_fields_del(
		&self,
		docid: &str,
		sheet_id: &str,
		field_ids: &Vec<String>,
	) {
		assert!(docid.is_empty() == false, "doc_id could not be empty");
		assert!(sheet_id.is_empty() == false, "sheet_id could not be empty");
		let token = self.get_access_token().await;
		let url = format!(
			"https://qyapi.weixin.qq.com/cgi-bin/wedoc/smartsheet/update_fields?access_token={token}"
		);
		#[derive(Debug, serde:: Deserialize)]
		struct DeleteFieldsResult {
			errcode: i32,
			errmsg: String,
		}
		let client = reqwest::Client::new();
		let ret = client
			.post(url)
			.json(&serde_json::json!({
				"docid":docid,
				"sheet_id":sheet_id,
				"field_ids":field_ids,
			}))
			.send()
			.await
			.unwrap();
		let ret = ret.text().await.unwrap();
		let ret = serde_json::from_str::<DeleteFieldsResult>(&ret).unwrap();
		log::debug!("update sheet fields result: {:?}", ret);
		assert!(
			ret.errcode == 0,
			"failed to update sheet fields: {}",
			ret.errmsg
		);
	}
}
