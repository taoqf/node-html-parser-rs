use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::smartsheet::Field;

#[derive(Debug, Clone)]
pub(crate) enum CellTextValue {
	/// 内容为文本(值为text)、内容为链接(值为url)
	Text(String),
	/// 单元格内容,
	Url((String, String)),
}

#[derive(Debug, Clone, serde::Serialize, serde:: Deserialize)]
pub(crate) struct CellImageValue {
	/// 图片 ID
	pub(crate) id: String,
	/// 图片标题
	pub(crate) title: String,
	/// 图片url
	pub(crate) image_url: String,
	/// 图片宽度
	pub(crate) width: i32,
	/// 图片高度
	pub(crate) height: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CellAttachmentFileType {
	/// 文件夹
	Folder,
	/// 微盘文件
	Wedrive,
	/// 收集表 30
	Collect,
	/// 文档 50
	Doc,
	/// 表格 51,
	Sheet,
	/// 幻灯片 52
	PPT,
	/// 思维导图 54
	MindMap,
	/// 流程图 55
	Flow,
	/// 智能表 70
	SmartSheet,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CellAttachmentDocType {
	/// 1
	Folder,
	/// 2
	File,
}

#[derive(Debug, Clone)]
pub(crate) struct CellAttachmentValue {
	/// 文件名
	pub(crate) name: String,
	/// 文件大小
	pub(crate) size: i32,
	/// 文件扩展名
	pub(crate) file_ext: String,
	/// 文件url
	pub(crate) file_url: String,
	/// 文件类型，文件夹为Folder，微盘文件为Wedrive，文件夹为Folder，微盘文件为Wedrive，收集表为30，文档为50，表格是51，幻灯片为52，思维导图为54，pub(crate) 流程图为55，智能表为70
	file_type: CellAttachmentFileType,
	/// 接口返回的文件类型，1为文件夹，2为文件
	pub(crate) doc_type: CellAttachmentDocType,
}

#[derive(Debug, Clone, serde::Serialize, serde:: Deserialize)]
pub(crate) struct CellUserValue {
	/// 成员ID
	pub(crate) user_id: Option<String>,
	/// 外部用户临时id，同一个用户在不同的智能表中返回的该id不一致。可进一步通过tmp_external_userid的转换接口转换成external_userid，方便识别外部用户的身份。
	pub(crate) tmp_external_userid: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde:: Deserialize)]
pub(crate) struct CellUrlValue {
	/// 填url
	#[serde(rename = "type")]
	pub(crate) value_type: String,
	/// 链接显示文本
	pub(crate) text: String,
	/// 链接跳转url
	pub(crate) link: String,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum OptionStyle {
	/// 1	浅红1
	浅红1 = 1,
	/// 2	浅橙1
	浅橙1,
	/// 3	浅天蓝1
	浅天蓝1,
	/// 4	浅绿1
	浅绿1,
	/// 5	浅紫1
	浅紫1,
	/// 6	浅粉红1
	浅粉红1,
	/// 7	浅灰1
	浅灰1,
	/// 8	白
	白,
	/// 9	灰
	灰,
	/// 10	浅蓝1
	浅蓝1,
	/// 11	浅蓝2
	浅蓝2,
	/// 12	蓝
	蓝,
	/// 13	浅天蓝2
	浅天蓝2,
	/// 14	天蓝
	天蓝,
	/// 15	浅绿2
	浅绿2,
	/// 16	绿
	绿,
	/// 17	浅红2
	浅红2,
	/// 18	红
	红,
	/// 19	浅橙2
	浅橙2,
	/// 20	橙
	橙,
	/// 21	浅黄1
	浅黄1,
	/// 22	浅黄2
	浅黄2,
	/// 23	黄
	黄,
	/// 24	浅紫2
	浅紫2,
	/// 25	紫
	紫,
	/// 26	浅粉红2
	浅粉红2,
	/// 27	粉红
	粉红,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OptionValue {
	/// 选项ID
	pub(crate) id: String,
	/// 选项颜色
	pub(crate) style: OptionStyle,
	/// 选项内容
	pub(crate) text: String,
}

#[derive(Debug, Clone)]
pub(crate) enum CellLocationSourceType {
	/// 1	腾讯地图
	Tencent,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct CellLocationValue {
	/// uint32	填1，表示来源为腾讯地图。目前只支持腾讯地图来源
	pub(crate) source_type: CellLocationSourceType,
	/// 地点ID
	pub(crate) id: String,
	/// 纬度
	pub(crate) latitude: String,
	/// 经度
	pub(crate) longitude: String,
	/// 地点名称
	pub(crate) title: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct CellTwoWayLinkRecordsValue {
	pub(crate) format: serde_json::Value, // todo 没有值
	pub(crate) text: String,
	#[serde(rename = "type")]
	pub(crate) text_type: String,
}

#[derive(Debug, Clone)]
pub(crate) struct CellAutoNumberValue {
	/// 序号
	pub(crate) seq: String,
	/// 展示的文本
	pub(crate) text: String,
}

#[derive(Debug, Clone)]
pub(crate) enum CellValue {
	FieldTypeUnknown,
	/// 文本(FIELD_TYPE_TEXT)	Object[](CellTextValue)
	FieldTypeText(CellTextValue),
	/// 数字(FIELD_TYPE_NUMBER)	double
	FieldTypeNumber(f64),
	/// 复选框(FIELD_TYPE_CHECKBOX)	bool
	FieldTypeCheckbox(bool),
	/// 日期(FIELD_TYPE_DATE_TIME)	string(以毫秒为单位的unix时间戳)
	FieldTypeDateTime(u64),
	/// 图片(FIELD_TYPE_IMAGE)	Object[](CellImageValue)
	FieldTypeImage(Vec<CellImageValue>),
	/// 文件(FIELD_TYPE_ATTACHMENT)	Object[](CellAttachmentValue)
	FieldTypeAttachment(Vec<CellAttachmentValue>),
	/// 成员(FIELD_TYPE_USER)	Object[](CellUserValue)
	FieldTypeUser(Vec<CellUserValue>),
	/// 链接(FIELD_TYPE_URL)	Object[](CellUrlValue)	数组类型为预留能力，目前只支持展示一个链接，建议只传入一个链接
	FieldTypeUrl(Vec<CellUrlValue>),
	/// 多选(FIELD_TYPE_SELECT)	Object[](OptionValue)
	FieldTypeSelect(Vec<OptionValue>),
	/// 进度(FIELD_TYPE_PROGRESS)	double
	FieldTypeProgress(f64),
	/// 电话(FIELD_TYPE_PHONE_NUMBER)	string
	FieldTypePhoneNumber(String),
	/// 邮箱(FIELD_TYPE_EMAIL)	string
	FieldTypeEmail(String),
	/// 单选(FIELD_TYPE_SINGLE_SELECT)	Object[](OptionValue)
	FieldTypeSingleSelect(Option<OptionValue>),
	/// 地理位置(FIELD_TYPE_LOCATION)	Object[](CellLocationValue)	长度不大于1的数组。
	FieldTypeLocation(Option<CellLocationValue>),
	/// 关联(FIELD_TYPE_REFERENCE)	string []	关联的记录id
	FieldTypeReference(Vec<String>),
	/// 双向关联(FIELD_TYPE_TWO_WAY_LINK)	Object[](CellTwoWayLinkRecordsValue)
	FieldTypeTwoWayLinkRecords(Vec<CellTwoWayLinkRecordsValue>),
	/// 货币(FIELD_TYPE_CURRENCY)	double
	FieldTypeCurrency(f64),
	/// 自动编号(FIELD_TYPE_AUTONUMBER)	Object[](CellAutoNumberValue)
	FieldTypeAutonumber(CellAutoNumberValue),
	/// 扩展字段 创建人
	FieldTypeExtCreateName(String),
	/// 扩展字段 创建时间
	FieldTypeExtCreateTime(u64),
	/// 扩展字段 最后编辑人
	FieldTypeExtUpdateName(String),
	/// 扩展字段 最后编辑时间
	FieldTypeExtUpdateTime(u64),
}

#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde:: Deserialize)]
pub(crate) struct ExtFields {
	pub(crate) creator_name: String,
	pub(crate) updater_name: String,
	pub(crate) tm_create: u64,
	pub(crate) tm_update: u64,
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 转换为数据对象
	pub(crate) fn record_val2obj(
		&self,
		values: &serde_json::Value,
		fields: &HashMap<String, Field>,
		ext_fields: &ExtFields,
	) -> HashMap<String, CellValue> {
		let values = values.as_object().unwrap();
		let values = fields
			.par_iter()
			.map(|(key, field)| {
				let v = values.get(key);
				match v {
					Some(val) => {
						let field_type = field.field_type.as_str();
						let value = match field_type {
							// 文本
							"FIELD_TYPE_TEXT" => {
								let val = match val.as_array() {
									Some(val) => val,
									None => &vec![],
								};
								let val = match val.len() {
									0 => CellValue::FieldTypeText(CellTextValue::Text(
										"".to_string(),
									)),
									_ => {
										let v = &val[0];
										let val_type = get_safe_str(v, "type");
										let val = match val_type.as_str() {
											"url" => {
												let link = get_safe_str(v, "link");
												let text = get_safe_str(v, "text");
												let val = (text, link);
												CellValue::FieldTypeText(CellTextValue::Url(val))
											}
											#[allow(unreachable_patterns)]
											_ | "text" => {
												let val = v.get("text").unwrap();
												CellValue::FieldTypeText(CellTextValue::Text(
													val.as_str().unwrap().to_string(),
												))
											}
										};
										val
									}
								};
								val
							}
							// 数字
							"FIELD_TYPE_NUMBER" => {
								let val = match val.as_f64() {
									Some(val) => CellValue::FieldTypeNumber(val),
									None => CellValue::FieldTypeNumber(0.0),
								};
								val
							}
							// 复选框
							"FIELD_TYPE_CHECKBOX" => {
								let val = match val.as_bool() {
									Some(val) => val,
									None => false,
								};
								CellValue::FieldTypeCheckbox(val)
							}
							// 日期
							"FIELD_TYPE_DATE_TIME" => {
								let val = val2str(val);
								let val = match val.parse() {
									Ok(val) => val,
									Err(_) => 0,
								};
								CellValue::FieldTypeDateTime(val)
							}
							// 图片
							"FIELD_TYPE_IMAGE" => {
								let val = val.as_array();
								let imgs = match val {
									Some(val) => val,
									None => &vec![],
								};
								let imgs = imgs
									.iter()
									.map(|img| {
										let height = get_safe_u64(img, "height") as i32;
										let width = get_safe_u64(img, "width") as i32;
										let id = get_safe_str(img, "id");
										let title = get_safe_str(img, "title");
										let image_url = get_safe_str(img, "image_url");
										CellImageValue {
											height,
											width,
											id,
											title,
											image_url,
										}
									})
									.collect();
								CellValue::FieldTypeImage(imgs)
							}
							// 文件
							"FIELD_TYPE_ATTACHMENT" => {
								let atts = match val.as_array() {
									None => &vec![],
									Some(val) => val,
								};
								let atts = atts
									.iter()
									.map(|it| {
										let it = it.as_object().unwrap();
										let name =
											it.get("name").unwrap().as_str().unwrap().to_owned();
										let size = it.get("size").unwrap().as_i64().unwrap() as i32;
										let file_ext = it
											.get("file_ext")
											.unwrap()
											.as_str()
											.unwrap()
											.to_owned();
										let file_url = it
											.get("file_url")
											.unwrap()
											.as_str()
											.unwrap()
											.to_owned();
										let file_type =
											it.get("file_type").unwrap().as_str().unwrap();
										let file_type = match file_type {
											"Folder" => CellAttachmentFileType::Folder,
											"Wedrive" => CellAttachmentFileType::Wedrive,
											"Collect" => CellAttachmentFileType::Collect,
											"Doc" => CellAttachmentFileType::Doc,
											"Sheet" => CellAttachmentFileType::Sheet,
											"52" => CellAttachmentFileType::PPT,
											"54" => CellAttachmentFileType::MindMap,
											"55" => CellAttachmentFileType::Flow,
											#[allow(unreachable_patterns)]
											_ | "70" => CellAttachmentFileType::SmartSheet,
										};
										let doc_type =
											it.get("doc_type").unwrap().as_i64().unwrap();
										let doc_type = match doc_type {
											1 => CellAttachmentDocType::Folder,
											#[allow(unreachable_patterns)]
											_ | 2 => CellAttachmentDocType::File,
										};

										CellAttachmentValue {
											name,
											size,
											file_ext,
											file_url,
											file_type,
											doc_type,
										}
									})
									.collect::<Vec<_>>();
								CellValue::FieldTypeAttachment(atts)
							}
							// 成员
							"FIELD_TYPE_USER" => {
								let users = match val.as_array() {
									Some(v) => v,
									None => &vec![],
								};
								let users = users
									.iter()
									.map(|it| serde_json::from_value(it.clone()).unwrap())
									.collect::<Vec<_>>();
								CellValue::FieldTypeUser(users)
							}
							// 超链接
							"FIELD_TYPE_URL" => {
								let urls = match val.as_array() {
									Some(v) => v,
									None => &vec![],
								};
								let urls = urls
									.iter()
									.map(|it| serde_json::from_value(it.clone()).unwrap())
									.collect::<Vec<_>>();
								CellValue::FieldTypeUrl(urls)
							}
							// 多选
							"FIELD_TYPE_SELECT" => {
								let vals = match val.as_array() {
									Some(v) => v,
									None => &vec![],
								};
								let vals = vals
									.iter()
									.map(|v| {
										let id = get_safe_str(v, "id");
										let text = get_safe_str(v, "text");
										let style = get_safe_u64(v, "style");
										let style = get_text_style(style);
										OptionValue { id, text, style }
									})
									.collect::<Vec<_>>();
								CellValue::FieldTypeSelect(vals)
							}
							// 创建人
							"FIELD_TYPE_CREATED_USER" => {
								CellValue::FieldTypeExtCreateName(ext_fields.creator_name.clone())
							}
							// 最后编辑人
							"FIELD_TYPE_MODIFIED_USER" => {
								CellValue::FieldTypeExtUpdateName(ext_fields.updater_name.clone())
							}
							// 创建时间
							"FIELD_TYPE_CREATED_TIME" => {
								CellValue::FieldTypeExtCreateTime(ext_fields.tm_create)
							}
							// 最后编辑时间
							"FIELD_TYPE_MODIFIED_TIME" => {
								CellValue::FieldTypeExtUpdateTime(ext_fields.tm_update)
							}
							// 进度
							"FIELD_TYPE_PROGRESS" => {
								let val = val2f64(val);
								CellValue::FieldTypeProgress(val)
							}
							// 电话
							"FIELD_TYPE_PHONE_NUMBER" => {
								let val = val2str(val);
								CellValue::FieldTypePhoneNumber(val)
							}
							// 邮件
							"FIELD_TYPE_EMAIL" => {
								let val = val2str(val);
								CellValue::FieldTypeEmail(val)
							}
							// 单选
							"FIELD_TYPE_SINGLE_SELECT" => {
								let vals = match val.as_array() {
									Some(v) => v,
									None => &vec![],
								};
								let val = if vals.len() == 0 {
									None
								} else {
									let v = &vals[0];
									let id = get_safe_str(v, "id");
									let text = get_safe_str(v, "text");
									let style = get_safe_u64(v, "style");
									let style = get_text_style(style);
									Some(OptionValue { id, text, style })
								};
								CellValue::FieldTypeSingleSelect(val)
							}
							// 关联1--文档
							"FIELD_TYPE_REFERENCE" => {
								log::debug!("fffffffffrrrrrrrrrrrrrrrr:{}", val);
								let vals = serde_json::from_value(val.clone()).unwrap();
								CellValue::FieldTypeReference(vals)
							}
							// 关联2--实际测试
							"FIELD_TYPE_TWOWAYLINKRECORDS" => {
								let vals = serde_json::from_value(val.clone());
								match vals {
									Ok(vals) => CellValue::FieldTypeTwoWayLinkRecords(vals),
									Err(..) => {
										let vals = serde_json::from_value(val.clone()).unwrap();
										CellValue::FieldTypeReference(vals)
									}
								}
							}
							// 地理位置
							"FIELD_TYPE_LOCATION" => {
								let vals = val.as_array();
								let vals = match vals {
									Some(v) => v,
									None => &vec![],
								};
								let val = if vals.len() == 0 {
									None
								} else {
									let v = &vals[0];
									let id = get_safe_str(v, "id");
									let title = get_safe_str(v, "title");
									let source_type = CellLocationSourceType::Tencent; // 文档中介绍，只支持腾讯地图
									let latitude = get_safe_str(v, "latitude");
									let longitude = get_safe_str(v, "longitude");
									Some(CellLocationValue {
										id,
										source_type,
										latitude,
										longitude,
										title,
									})
								};
								CellValue::FieldTypeLocation(val)
							}
							// 公式
							"FIELD_TYPE_FORMULA" => {
								// !!! 公式类型实际上不能获得值 公式表达式目前是不支持获取内容的，可以调用get_fields获取表达式，再根据表达式获得 @see https://developer.work.weixin.qq.com/community/question/detail?content_id=16633672476770750488 实际上，公式连字段描述获取的全都是空
								CellValue::FieldTypeText(CellTextValue::Text("".to_string()))
							}
							// 货币
							"FIELD_TYPE_CURRENCY" => {
								let val = val2f64(val);
								CellValue::FieldTypeCurrency(val)
							}
							// 群
							// "FIELD_TYPE_WWGROUP" => {}
							// 自动编号
							"FIELD_TYPE_AUTONUMBER" => {
								let seq = get_safe_str(val, "seq");
								let text = get_safe_str(val, "text");
								CellValue::FieldTypeAutonumber(CellAutoNumberValue {
									seq: seq.to_owned(),
									text: text.to_owned(),
								})
							}
							#[allow(unreachable_patterns)]
							_ => {
								log::error!("Unknow field type: {}{}{:#?}", key, val, field);
								CellValue::FieldTypeUnknown
							}
						};
						(key.to_owned(), value)
					}
					None => {
						let field_type = field.field_type.as_str();
						let value = match field_type {
							// 创建人
							"FIELD_TYPE_CREATED_USER" => {
								CellValue::FieldTypeExtCreateName(ext_fields.creator_name.clone())
							}
							// 最后编辑人
							"FIELD_TYPE_MODIFIED_USER" => {
								CellValue::FieldTypeExtUpdateName(ext_fields.updater_name.clone())
							}
							// 创建时间
							"FIELD_TYPE_CREATED_TIME" => {
								CellValue::FieldTypeExtCreateTime(ext_fields.tm_create)
							}
							// 最后编辑时间
							"FIELD_TYPE_MODIFIED_TIME" => {
								CellValue::FieldTypeExtUpdateTime(ext_fields.tm_update)
							}
							#[allow(unreachable_patterns)]
							_ => CellValue::FieldTypeUnknown,
						};
						(key.to_owned(), value)
					}
				}
			})
			.collect::<std::collections::HashMap<_, _>>();
		return values;
	}
}

fn val2str(val: &serde_json::Value) -> String {
	let val = match val.as_str() {
		Some(v) => v.to_owned(),
		None => "".to_owned(),
	};
	return val;
}

fn val2f64(val: &serde_json::Value) -> f64 {
	let val = match val.as_f64() {
		Some(v) => v,
		None => 0.0,
	};
	return val;
}

fn val2i64(val: &serde_json::Value) -> i64 {
	let val = match val.as_i64() {
		Some(v) => v,
		None => 0,
	};
	return val;
}

fn val2bool(val: &serde_json::Value) -> bool {
	let val = match val.as_bool() {
		Some(v) => v,
		None => false,
	};
	return val;
}

fn val2arr(val: &serde_json::Value) -> Vec<String> {
	let val = match val.as_array() {
		Some(v) => v.iter().map(val2str).collect::<Vec<_>>(),
		None => vec![val2str(val)],
	};
	return val;
}

fn get_safe_u64(val: &serde_json::Value, key: &str) -> u64 {
	let val = match val.get(key) {
		Some(v) => match v.as_u64() {
			Some(v) => v.to_owned(),
			None => 0,
		},
		None => 0,
	};
	return val;
}

#[allow(dead_code)]
fn get_safe_f64(val: &serde_json::Value, key: &str) -> f64 {
	let val = match val.get(key) {
		Some(v) => match v.as_f64() {
			Some(v) => v.to_owned(),
			None => 0.0,
		},
		None => 0.0,
	};
	return val;
}

fn get_safe_str(val: &serde_json::Value, key: &str) -> String {
	let val = match val.get(key) {
		Some(v) => match v.as_str() {
			Some(v) => v.to_owned(),
			None => "".to_owned(),
		},
		None => "".to_owned(),
	};
	return val;
}

fn get_text_style(style: u64) -> OptionStyle {
	let style = match style {
		2 => OptionStyle::浅橙1,
		3 => OptionStyle::浅天蓝1,
		4 => OptionStyle::浅绿1,
		5 => OptionStyle::浅紫1,
		6 => OptionStyle::浅粉红1,
		7 => OptionStyle::浅灰1,
		8 => OptionStyle::白,
		9 => OptionStyle::灰,
		10 => OptionStyle::浅蓝1,
		11 => OptionStyle::浅蓝2,
		12 => OptionStyle::蓝,
		13 => OptionStyle::浅天蓝2,
		14 => OptionStyle::天蓝,
		15 => OptionStyle::浅绿2,
		16 => OptionStyle::绿,
		17 => OptionStyle::浅红2,
		18 => OptionStyle::红,
		19 => OptionStyle::浅橙2,
		20 => OptionStyle::橙,
		21 => OptionStyle::浅黄1,
		22 => OptionStyle::浅黄2,
		23 => OptionStyle::黄,
		24 => OptionStyle::浅紫2,
		25 => OptionStyle::紫,
		26 => OptionStyle::浅粉红2,
		27 => OptionStyle::粉红,
		#[allow(unreachable_patterns)]
		_ | 1 => OptionStyle::浅红1,
	};
	return style;
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 数据对象转换为json对象
	pub(crate) fn record_obj2val(&self, values: &HashMap<&str, &CellValue>) -> serde_json::Value {
		let values = values
			.iter()
			.map(|(key, val)| {
				log::debug!("00000000000000000000000000000{}:{:#?}", key, val);
				let val = match val {
					// 文本
					CellValue::FieldTypeText(val) => match val {
						CellTextValue::Text(val) => {
							serde_json::json!({
								"text": val
							})
						}
						CellTextValue::Url((text, link)) => {
							serde_json::json!({
								"text": text,
								"link": link
							})
						}
					},
					// 数字
					CellValue::FieldTypeNumber(val) => serde_json::json!(val),
					// 复选框
					CellValue::FieldTypeCheckbox(val) => serde_json::json!(val),
					// 日期
					CellValue::FieldTypeDateTime(val) => serde_json::json!(val.to_string()),
					// 图片
					CellValue::FieldTypeImage(imgs) => serde_json::Value::Array(
						imgs.iter()
							.map(|img| {
								serde_json::json!({
									"height": img.height,
									"width": img.width,
									"id": img.id,
									"title": img.title,
									"image_url": img.image_url,
								})
							})
							.collect(),
					),
					// 文件
					CellValue::FieldTypeAttachment(files) => serde_json::Value::Array(
						files
							.iter()
							.map(|file| {
								let file_type = match file.file_type {
									CellAttachmentFileType::Folder => "Folder",
									CellAttachmentFileType::Wedrive => "Wedrive",
									CellAttachmentFileType::Collect => "Collect",
									CellAttachmentFileType::Doc => "Doc",
									CellAttachmentFileType::Sheet => "Sheet",
									CellAttachmentFileType::PPT => "52",
									CellAttachmentFileType::MindMap => "54",
									CellAttachmentFileType::Flow => "55",
									CellAttachmentFileType::SmartSheet => "70",
								};
								let doc_type = match file.doc_type {
									CellAttachmentDocType::File => 2,
									CellAttachmentDocType::Folder => 1,
								};
								serde_json::json!({
									"file_type": file_type,
									"doc_type": doc_type,
									"name": file.name,
									"size": file.size,
									"file_ext": file.file_ext,
									"file_url": file.file_url,
								})
							})
							.collect(),
					),
					// 成员
					CellValue::FieldTypeUser(users) => serde_json::Value::Array(
						users
							.iter()
							.map(|user| serde_json::to_value(user).unwrap())
							.collect(),
					),
					// 超链接
					CellValue::FieldTypeUrl(urls) => serde_json::Value::Array(
						urls.iter()
							.map(|url| serde_json::to_value(url).unwrap())
							.collect(),
					),
					// 多选
					CellValue::FieldTypeSelect(vals) => serde_json::Value::Array(
						vals.iter()
							.map(|val| {
								serde_json::json!({
									"id": val.id,
									"text": val.text,
									"style": get_text_style_num(&val.style),
								})
							})
							.collect(),
					),
					// 创建人
					// 最后编辑人
					// 创建时间
					// 最后编辑时间
					// 进度
					CellValue::FieldTypeProgress(val) => serde_json::json!(val),
					// 电话
					CellValue::FieldTypePhoneNumber(val) => serde_json::json!(val),
					// 邮件
					CellValue::FieldTypeEmail(val) => serde_json::json!(val),
					// 单选
					CellValue::FieldTypeSingleSelect(val) => match val {
						Some(val) => serde_json::Value::Array(vec![serde_json::json!({
							"id": val.id,
							"text": val.text,
							"style": get_text_style_num(&val.style),
						})]),
						None => serde_json::Value::Array(vec![]),
					},
					// 关联
					CellValue::FieldTypeReference(vals) => serde_json::Value::Array(
						vals.iter()
							.map(|val| serde_json::to_value(val).unwrap())
							.collect(),
					),
					// 双向关联
					CellValue::FieldTypeTwoWayLinkRecords(vals) => serde_json::Value::Array(
						vals.iter()
							.map(|val| {
								serde_json::json!({
									// "format": {},
									"text": val.text,
									"type": val.text_type,
								})
							})
							.collect(),
					),
					// 地理位置
					CellValue::FieldTypeLocation(val) => match val {
						Some(val) => serde_json::Value::Array(vec![serde_json::json!({
							"id": val.id,
							"title": val.title,
							"source_type": 1, // 文档中介绍，只支持腾讯地图
							"latitude": val.latitude,
							"longitude": val.longitude,
						})]),
						None => serde_json::Value::Array(vec![]),
					},
					// 公式
					// 货币
					CellValue::FieldTypeCurrency(val) => serde_json::json!(val),
					// 群
					// 自动编号
					CellValue::FieldTypeAutonumber(val) => serde_json::json!({
						"seq": val.seq,
						"text": val.text,
					}),
					_ => serde_json::json!(null),
				};
				let mut json = serde_json::map::Map::new();
				json.insert(key.to_string(), val);
				serde_json::Value::Object(json)
			})
			.collect::<Vec<_>>();
		let values = serde_json::Value::Array(values);
		return values;
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	/// 数据对象转换为json对象
	pub(crate) fn record_obj2val_by_type(
		&self,
		values: &HashMap<&str, &CellValue>,
		fields: &HashMap<String, Field>,
	) -> serde_json::Value {
		let values = values
			.iter()
			.map(|(key, val)| {
				let key = key.to_string();
				match fields.get(&key) {
					None => {
						log::debug!(
							"11111111111111111111111111111{}:{:#?}=={:#?}",
							key,
							val,
							fields
						);
						(key, serde_json::json!(null))
					}
					Some(field) => {
						log::debug!(
							"00000000000000000000000000000{}:{:#?}=={:#?}",
							key,
							val,
							field
						);
						let val = match field.field_type.as_str() {
							// 文本
							"FIELD_TYPE_TEXT" => {
								let val = self.record_val2str(val);
								serde_json::json!([{
									"type": "text",
									"text": val,
									"link": ""
								}])
							}
							// 数字
							"FIELD_TYPE_NUMBER" => {
								let val = self.record_val2f64(val);
								serde_json::json!(val)
							}
							// 复选框
							"FIELD_TYPE_CHECKBOX" => {
								let val = self.record_val2bool(val);
								serde_json::json!(val)
							}
							// 日期
							"FIELD_TYPE_DATE_TIME" => {
								let val = self.record_val2f64(val) as i64;
								serde_json::json!(val.to_string())
							}
							// 图片
							"FIELD_TYPE_IMAGE" => {
								let val = match val {
									CellValue::FieldTypeImage(vals) => vals
										.iter()
										.map(|val| {
											serde_json::json!({
												"id": val.id,
												"title": val.title,
												"image_url": val.image_url,
												"width": val.width,
												"height": val.height,
											})
										})
										.collect::<Vec<_>>(),
									_ => vec![],
								};
								serde_json::json!(val)
							}
							// 文件
							"FIELD_TYPE_ATTACHMENT" => {
								let val = match val {
									CellValue::FieldTypeAttachment(vals) => vals
										.iter()
										.map(|val| {
											let file_type = match val.file_type {
												CellAttachmentFileType::Folder => "Folder",
												CellAttachmentFileType::Wedrive => "Wedrive",
												CellAttachmentFileType::Collect => "30",
												CellAttachmentFileType::Doc => "50",
												CellAttachmentFileType::Sheet => "51",
												CellAttachmentFileType::PPT => "52",
												CellAttachmentFileType::MindMap => "54",
												CellAttachmentFileType::Flow => "55",
												CellAttachmentFileType::SmartSheet => "70",
											};
											let doc_type = match val.doc_type {
												CellAttachmentDocType::Folder => 1,
												CellAttachmentDocType::File => 2,
											};
											serde_json::json!({
												"name": val.name,
												"size": val.size,
												"file_ext": val.file_ext,
												"file_type": file_type,
												"doc_type": doc_type,
											})
										})
										.collect::<Vec<_>>(),
									_ => vec![],
								};
								serde_json::json!(val)
							}
							// 成员
							"FIELD_TYPE_USER" => {
								let val = match val {
									CellValue::FieldTypeUser(vals) => vals
										.iter()
										.map(|val| {
											serde_json::json!({
												"user_id": val.user_id,
												"tmp_external_userid": val.tmp_external_userid,
											})
										})
										.collect::<Vec<_>>(),
									_ => vec![],
								};
								serde_json::json!(val)
							}
							// 超链接
							"FIELD_TYPE_URL" => {
								let val = match val {
									CellValue::FieldTypeUrl(vals) => vals
										.iter()
										.map(|val| {
											serde_json::json!({
												"type": val.value_type,
												"text": val.text,
												"link": val.link,
											})
										})
										.collect::<Vec<_>>(),
									_ => vec![],
								};
								serde_json::json!(val)
							}
							// 多选
							"FIELD_TYPE_SELECT" => {
								let val = match val {
									CellValue::FieldTypeSelect(vals) => vals
										.iter()
										.map(|val| {
											let style = get_text_style_num(&val.style);
											serde_json::json!({
												// "id": val.id,
												"text": val.text,
												"style": style
											})
										})
										.collect::<Vec<_>>(),
									_ => vec![],
								};
								serde_json::json!(val)
							}
							// 创建人
							// "FIELD_TYPE_CREATED_USER" => {}
							// 最后编辑人
							// "FIELD_TYPE_MODIFIED_USER" => {}
							// 创建时间
							// "FIELD_TYPE_CREATED_TIME" => {}
							// 最后编辑时间
							// "FIELD_TYPE_MODIFIED_TIME" => {}
							// 进度
							"FIELD_TYPE_PROGRESS" => {
								let val = match val {
									CellValue::FieldTypeProgress(val) => *val,
									_ => 0.0,
								};
								serde_json::json!(val)
							}
							// 电话
							"FIELD_TYPE_PHONE_NUMBER" => {
								let val = match val {
									CellValue::FieldTypePhoneNumber(val) => val,
									_ => "",
								};
								serde_json::json!(val)
							}
							// 邮件
							"FIELD_TYPE_EMAIL" => {
								let val = match val {
									CellValue::FieldTypeEmail(val) => val,
									_ => "",
								};
								serde_json::json!(val)
							}
							// 单选
							"FIELD_TYPE_SINGLE_SELECT" => {
								let val = self.record_val2str(val);
								let val = vec![serde_json::json!({
									// "id": val.id,
									"text": val,
									"style": 1,
								})];
								serde_json::json!(val)
							}
							// 关联1--文档
							"FIELD_TYPE_REFERENCE" => {
								let val = match val {
									CellValue::FieldTypeReference(val) => val,
									_ => &vec![],
								};
								serde_json::json!(val)
							}
							// 关联2--实际测试
							"FIELD_TYPE_TWOWAYLINKRECORDS" => {
								let val = match val {
									CellValue::FieldTypeReference(val) => serde_json::json!(val),
									CellValue::FieldTypeTwoWayLinkRecords(val) => {
										serde_json::json!(val)
									}
									_ => serde_json::json!([]),
								};
								serde_json::json!(val)
							}
							// 地理位置
							"FIELD_TYPE_LOCATION" => {
								let val = match val {
									CellValue::FieldTypeLocation(vals) => vals
										.iter()
										.map(|val| {
											let source_type = 1; // 文档中介绍，只支持腾讯地图
											serde_json::json!({
												"id": val.id,
												"title": val.title,
												"source_type": source_type,
												"latitude": val.latitude,
												"longitude": val.title,
											})
										})
										.collect::<Vec<_>>(),
									_ => vec![],
								};
								serde_json::json!(val)
							}
							// 公式
							// "FIELD_TYPE_FORMULA" => {}
							// 货币
							"FIELD_TYPE_CURRENCY" => {
								let val = match val {
									CellValue::FieldTypeCurrency(val) => val,
									_ => &0.0,
								};
								serde_json::json!(val)
							}
							// 群
							// "FIELD_TYPE_WWGROUP" => {}
							// 自动编号
							"FIELD_TYPE_AUTONUMBER" => {
								let val = match val {
									CellValue::FieldTypeAutonumber(val) => serde_json::json!({
										"seq": val.seq,
										"text": val.text,
									}),
									_ => serde_json::json!(null),
								};
								serde_json::json!(val)
							}
							#[allow(unreachable_patterns)]
							_ => {
								log::error!("Unknow field type: {}{:#?}{:#?}", key, val, field);
								serde_json::json!(null)
							}
						};
						(key, val)
					}
				}
			})
			.collect::<serde_json::Map<_, _>>();

		return serde_json::json!(values);
	}
}

fn get_text_style_num(style: &OptionStyle) -> i32 {
	let style = match style {
		OptionStyle::浅橙1 => 2,
		OptionStyle::浅天蓝1 => 3,
		OptionStyle::浅绿1 => 4,
		OptionStyle::浅紫1 => 5,
		OptionStyle::浅粉红1 => 6,
		OptionStyle::浅灰1 => 7,
		OptionStyle::白 => 8,
		OptionStyle::灰 => 9,
		OptionStyle::浅蓝1 => 10,
		OptionStyle::浅蓝2 => 11,
		OptionStyle::蓝 => 12,
		OptionStyle::浅天蓝2 => 13,
		OptionStyle::天蓝 => 14,
		OptionStyle::浅绿2 => 15,
		OptionStyle::绿 => 16,
		OptionStyle::浅红2 => 17,
		OptionStyle::红 => 18,
		OptionStyle::浅橙2 => 19,
		OptionStyle::橙 => 20,
		OptionStyle::浅黄1 => 21,
		OptionStyle::浅黄2 => 22,
		OptionStyle::黄 => 23,
		OptionStyle::浅紫2 => 24,
		OptionStyle::紫 => 25,
		OptionStyle::浅粉红2 => 26,
		OptionStyle::粉红 => 27,
		OptionStyle::浅红1 => 1,
	};
	return style;
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	pub(crate) fn record_val2str(&self, value: &CellValue) -> String {
		let val = match value {
			// 文本
			CellValue::FieldTypeText(val) => match val {
				CellTextValue::Text(val) => val.to_owned(),
				CellTextValue::Url((_text, link)) => link.to_owned(),
			},
			// 数字
			CellValue::FieldTypeNumber(val) => val.to_string(),
			// 复选框
			CellValue::FieldTypeCheckbox(val) => val.to_string(),
			// 日期
			CellValue::FieldTypeDateTime(val) => val.to_string(),
			// 图片
			CellValue::FieldTypeImage(_imgs) => "".to_owned(),
			// 文件
			CellValue::FieldTypeAttachment(_files) => "".to_owned(),
			// 成员
			CellValue::FieldTypeUser(users) => {
				if let Some(user) = users.get(0) {
					match &user.user_id {
						Some(id) => id.to_string(),
						None => "".to_owned(),
					}
				} else {
					return "".to_owned();
				}
			}
			// 超链接
			CellValue::FieldTypeUrl(urls) => {
				if let Some(url) = urls.get(0) {
					url.link.to_string()
				} else {
					return "".to_owned();
				}
			}
			// 多选
			CellValue::FieldTypeSelect(vals) => {
				if let Some(val) = vals.get(0) {
					val.text.to_string()
				} else {
					return "".to_owned();
				}
			}
			// 创建人
			CellValue::FieldTypeExtCreateName(val) => val.clone(),
			// 最后编辑人
			CellValue::FieldTypeExtUpdateName(val) => val.clone(),
			// 创建时间
			CellValue::FieldTypeExtCreateTime(dt) => {
				crate::atoms::dt::stamp2str(*dt, crate::atoms::dt::DtType::DATETIME)
			}
			// 最后编辑时间
			CellValue::FieldTypeExtUpdateTime(dt) => {
				crate::atoms::dt::stamp2str(*dt, crate::atoms::dt::DtType::DATETIME)
			}
			// 进度
			CellValue::FieldTypeProgress(val) => val.to_string(),
			// 电话
			CellValue::FieldTypePhoneNumber(val) => val.to_string(),
			// 邮件
			CellValue::FieldTypeEmail(val) => val.to_string(),
			// 单选
			CellValue::FieldTypeSingleSelect(val) => match val {
				Some(val) => val.text.to_string(),
				None => "".to_owned(),
			},
			// 关联
			CellValue::FieldTypeReference(vals) => {
				if let Some(val) = vals.get(0) {
					val.to_string()
				} else {
					return "".to_owned();
				}
			}
			// 双向关联
			CellValue::FieldTypeTwoWayLinkRecords(vals) => {
				if let Some(val) = vals.get(0) {
					val.text.clone()
				} else {
					return "".to_owned();
				}
			}
			// 地理位置
			CellValue::FieldTypeLocation(val) => match val {
				Some(val) => val.title.to_string(),
				None => "".to_string(),
			},
			// 公式
			// 货币
			CellValue::FieldTypeCurrency(val) => val.to_string(),
			// 群
			// 自动编号
			CellValue::FieldTypeAutonumber(val) => val.text.to_string(),
			_ => "".to_owned(),
		};
		return val;
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	pub(crate) fn record_val2f64(&self, value: &CellValue) -> f64 {
		let val = match value {
			// 文本
			CellValue::FieldTypeText(val) => {
				let val = match val {
					CellTextValue::Text(val) => val,
					CellTextValue::Url((text, _link)) => text,
				};
				match val.parse() {
					Err(_) => 0.0,
					Ok(val) => val,
				}
			}
			// 数字
			CellValue::FieldTypeNumber(val) => *val,
			// 复选框
			CellValue::FieldTypeCheckbox(val) => match val {
				true => 1.0,
				false => 0.0,
			},
			// 日期
			CellValue::FieldTypeDateTime(val) => *val as f64,
			// 图片
			CellValue::FieldTypeImage(_imgs) => 0.0,
			// 文件
			CellValue::FieldTypeAttachment(_files) => 0.0,
			// 成员
			CellValue::FieldTypeUser(_users) => 0.0,
			// 超链接
			CellValue::FieldTypeUrl(_urls) => 0.0,
			// 多选
			CellValue::FieldTypeSelect(_vals) => 0.0,
			// 创建人
			CellValue::FieldTypeExtCreateName(val) => val.parse().unwrap_or_default(),
			// 最后编辑人
			CellValue::FieldTypeExtUpdateName(val) => val.parse().unwrap_or_default(),
			// 创建时间
			CellValue::FieldTypeExtCreateTime(dt) => *dt as f64,
			// 最后编辑时间
			CellValue::FieldTypeExtUpdateTime(dt) => *dt as f64,

			// 进度
			CellValue::FieldTypeProgress(val) => *val,
			// 电话
			CellValue::FieldTypePhoneNumber(val) => match val.parse() {
				Err(_) => 0.0,
				Ok(val) => val,
			},
			// 邮件
			CellValue::FieldTypeEmail(_val) => 0.0,
			// 单选
			CellValue::FieldTypeSingleSelect(val) => match val {
				Some(val) => match val.text.parse() {
					Err(_) => 0.0,
					Ok(val) => val,
				},
				None => 0.0,
			},
			// 关联
			CellValue::FieldTypeReference(_vals) => 0.0,
			// 双向关联
			CellValue::FieldTypeTwoWayLinkRecords(_vals) => 0.0,
			// 地理位置
			CellValue::FieldTypeLocation(val) => match val {
				Some(val) => match val.title.parse() {
					Err(_) => 0.0,
					Ok(val) => val,
				},
				None => 0.0,
			},
			// 公式
			// 货币
			CellValue::FieldTypeCurrency(val) => *val,
			// 群
			// 自动编号
			CellValue::FieldTypeAutonumber(val) => match val.seq.parse() {
				Err(_) => 0.0,
				Ok(val) => val,
			},
			_ => 0.0,
		};
		return val;
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	pub(crate) fn record_val2arr(&self, value: &CellValue) -> Vec<String> {
		let val = match value {
			// 文本
			CellValue::FieldTypeText(val) => match val {
				CellTextValue::Text(val) => vec![val.to_string()],
				CellTextValue::Url((text, _link)) => vec![text.to_string()],
			},
			// 数字
			CellValue::FieldTypeNumber(val) => vec![val.to_string()],
			// 复选框
			CellValue::FieldTypeCheckbox(val) => match val {
				true => vec!["true".to_owned()],
				false => vec![],
			},
			// 日期
			CellValue::FieldTypeDateTime(val) => vec![val.to_string()],
			// 图片
			CellValue::FieldTypeImage(_imgs) => vec![],
			// 文件
			CellValue::FieldTypeAttachment(_files) => vec![],
			// 成员
			CellValue::FieldTypeUser(_users) => vec![],
			// 超链接
			CellValue::FieldTypeUrl(_urls) => vec![],
			// 多选
			CellValue::FieldTypeSelect(_vals) => vec![],
			// 创建人
			CellValue::FieldTypeExtCreateName(val) => vec![val.clone()],
			// 最后编辑人
			CellValue::FieldTypeExtUpdateName(val) => vec![val.clone()],
			// 创建时间
			CellValue::FieldTypeExtCreateTime(dt) => {
				vec![crate::atoms::dt::stamp2str(
					*dt,
					crate::atoms::dt::DtType::DATETIME,
				)]
			}
			// 最后编辑时间
			CellValue::FieldTypeExtUpdateTime(dt) => {
				vec![crate::atoms::dt::stamp2str(
					*dt,
					crate::atoms::dt::DtType::DATETIME,
				)]
			}
			// 进度
			CellValue::FieldTypeProgress(val) => vec![val.to_string()],
			// 电话
			CellValue::FieldTypePhoneNumber(val) => vec![val.to_string()],
			// 邮件
			CellValue::FieldTypeEmail(_val) => vec![],
			// 单选
			CellValue::FieldTypeSingleSelect(val) => match val {
				Some(val) => vec![val.text.to_owned()],
				None => vec![],
			},
			// 关联
			CellValue::FieldTypeReference(vals) => vals.iter().map(|r| r.to_string()).collect(),
			// 双向关联
			CellValue::FieldTypeTwoWayLinkRecords(vals) => {
				vals.iter().map(|r| r.text.to_owned()).collect()
			}
			// 地理位置
			CellValue::FieldTypeLocation(val) => match val {
				Some(val) => vec![val.title.to_owned()],
				None => vec![],
			},
			// 公式
			// 货币
			CellValue::FieldTypeCurrency(val) => vec![val.to_string()],
			// 群
			// 自动编号
			CellValue::FieldTypeAutonumber(val) => vec![val.text.to_owned()],
			_ => vec![],
		};
		return val;
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	pub(crate) fn record_val2bool(&self, value: &CellValue) -> bool {
		let val = match value {
			// 文本
			CellValue::FieldTypeText(val) => match val {
				CellTextValue::Text(val) => val.is_empty(),
				CellTextValue::Url((text, _link)) => text.is_empty(),
			},
			// 数字
			CellValue::FieldTypeNumber(val) => *val == 0.0,
			// 复选框
			CellValue::FieldTypeCheckbox(val) => *val,
			// 日期
			CellValue::FieldTypeDateTime(val) => *val == 0,
			// 图片
			CellValue::FieldTypeImage(imgs) => imgs.is_empty(),
			// 文件
			CellValue::FieldTypeAttachment(files) => files.is_empty(),
			// 成员
			CellValue::FieldTypeUser(users) => users.is_empty(),
			// 超链接
			CellValue::FieldTypeUrl(urls) => urls.is_empty(),
			// 多选
			CellValue::FieldTypeSelect(vals) => vals.is_empty(),
			// 创建人
			CellValue::FieldTypeExtCreateName(val) => val.is_empty(),
			// 最后编辑人
			CellValue::FieldTypeExtUpdateName(val) => val.is_empty(),
			// 创建时间
			CellValue::FieldTypeExtCreateTime(_dt) => true,
			// 最后编辑时间
			CellValue::FieldTypeExtUpdateTime(_dt) => true,

			// 进度
			CellValue::FieldTypeProgress(val) => *val == 0.0,
			// 电话
			CellValue::FieldTypePhoneNumber(val) => val.is_empty(),
			// 邮件
			CellValue::FieldTypeEmail(val) => val.is_empty(),
			// 单选
			CellValue::FieldTypeSingleSelect(val) => val.is_some(),
			// 关联
			CellValue::FieldTypeReference(vals) => vals.is_empty(),
			// 双向关联
			CellValue::FieldTypeTwoWayLinkRecords(vals) => vals.is_empty(),
			// 地理位置
			CellValue::FieldTypeLocation(val) => val.is_some(),
			// 公式
			// 货币
			CellValue::FieldTypeCurrency(val) => *val == 0.0,
			// 群
			// 自动编号
			CellValue::FieldTypeAutonumber(val) => val.text.is_empty(),
			_ => false,
		};
		return val;
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	pub(crate) fn record_val_compare(
		&self,
		value: &CellValue,
		compared_value: &serde_json::Value,
		op: &str,
	) -> bool {
		enum CompareType {
			Eq,
			Neq,
			Gt,
			Gte,
			Lt,
			Lte,
			In,
		}
		let op = match op {
			"eq" | "EQ" | "=" | "==" | "===" => CompareType::Eq,
			"neq" | "NEQ" | "!=" | "!==" | "<>" => CompareType::Neq,
			"gt" | ">" => CompareType::Gt,
			"gte" | ">=" => CompareType::Gte,
			"lt" | "<" => CompareType::Lt,
			"lte" | "<=" => CompareType::Lte,
			"in" | "contains" | "like" => CompareType::In,
			_ => {
				panic!("不支持的操作符: {}", op)
			}
		};

		let val = match value {
			// 文本
			CellValue::FieldTypeText(val) => {
				let val = match val {
					CellTextValue::Text(val) => val.to_owned(),
					CellTextValue::Url((_text, link)) => link.to_owned(),
				};
				let val_compare = val2str(compared_value);
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val.contains(val_compare.as_str()),
				}
			}
			// 数字
			CellValue::FieldTypeNumber(val) => {
				let val = *val;
				let val_compare = val2f64(compared_value);
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => false,
				}
			}
			// 复选框
			CellValue::FieldTypeCheckbox(val) => {
				let val = *val;
				let val_compare = val2bool(compared_value);
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => false,
					CompareType::Gte => false,
					CompareType::Lt => false,
					CompareType::Lte => false,
					CompareType::In => false,
				}
			}
			// 日期
			CellValue::FieldTypeDateTime(val) => {
				let val = *val as i64;
				let val_compare = val2i64(compared_value);
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => false,
				}
			}
			// 图片
			CellValue::FieldTypeImage(vals) => {
				let val = match vals.get(0) {
					Some(val) => val.title.to_owned(),
					None => "".to_owned(),
				};
				let val_compares = val2arr(compared_value);
				let val_compare = match val_compares.get(0) {
					Some(val) => val.to_owned(),
					None => "".to_owned(),
				};
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val_compares.contains(&val),
				}
			}
			// 文件
			CellValue::FieldTypeAttachment(vals) => {
				let val = match vals.get(0) {
					Some(val) => val.name.to_owned(),
					None => "".to_owned(),
				};
				let val_compares = val2arr(compared_value);
				let val_compare = match val_compares.get(0) {
					Some(val) => val.to_owned(),
					None => "".to_owned(),
				};
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val_compares.contains(&val),
				}
			}
			// 成员
			CellValue::FieldTypeUser(vals) => {
				let val = match vals.get(0) {
					Some(val) => match &val.user_id {
						Some(val) => val.to_owned(),
						None => "".to_owned(),
					},
					None => "".to_owned(),
				};
				let val_compares = val2arr(compared_value);
				let val_compare = match val_compares.get(0) {
					Some(val) => val.to_owned(),
					None => "".to_owned(),
				};
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val_compares.contains(&val),
				}
			}
			// 超链接
			CellValue::FieldTypeUrl(vals) => {
				let val = match vals.get(0) {
					Some(val) => val.text.to_owned(),
					None => "".to_owned(),
				};
				let val_compares = val2arr(compared_value);
				let val_compare = match val_compares.get(0) {
					Some(val) => val.to_owned(),
					None => "".to_owned(),
				};
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val_compares.contains(&val),
				}
			}
			// 多选
			CellValue::FieldTypeSelect(vals) => {
				let val = match vals.get(0) {
					Some(val) => val.text.to_owned(),
					None => "".to_owned(),
				};
				let val_compares = val2arr(compared_value);
				let val_compare = match val_compares.get(0) {
					Some(val) => val.to_owned(),
					None => "".to_owned(),
				};
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val_compares.contains(&val),
				}
			}
			// 创建人 最后编辑人
			CellValue::FieldTypeExtCreateName(val) | CellValue::FieldTypeExtUpdateName(val) => {
				let val = val.clone();
				let val_compare = val2str(compared_value);
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val_compare.contains(&val),
				}
			}
			// 创建时间 最后编辑时间
			CellValue::FieldTypeExtCreateTime(dt) | CellValue::FieldTypeExtUpdateTime(dt) => {
				let val = *dt;
				let val_compare = val2i64(compared_value) as u64;
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte | CompareType::In => val <= val_compare,
				}
			}

			// 进度 货币
			CellValue::FieldTypeProgress(val) | CellValue::FieldTypeCurrency(val) => {
				let val = *val;
				let val_compare = val2f64(compared_value);
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => false,
				}
			}
			// 电话 邮件
			CellValue::FieldTypePhoneNumber(val) | CellValue::FieldTypeEmail(val) => {
				let val = val.to_owned();
				let val_compare = val2str(compared_value);
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val.contains(val_compare.as_str()),
				}
			}
			// 邮件
			// CellValue::FieldTypeEmail(val) => {
			// 	let val = *val;
			// 	let val_compare = val2str(compared_value);
			// 	match op {
			// 		CompareType::Eq => val == val_compare,
			// 		CompareType::Neq => val != val_compare,
			// 		CompareType::Gt => val > val_compare,
			// 		CompareType::Gte => val >= val_compare,
			// 		CompareType::Lt => val < val_compare,
			// 		CompareType::Lte => val <= val_compare,
			// 		CompareType::In => val.contains(val_compare.as_str()),
			// 		_ => false,
			// 	}
			// }
			// 单选
			CellValue::FieldTypeSingleSelect(val) => {
				let val = match val {
					Some(val) => val.text.to_string(),
					None => "".to_owned(),
				};
				let val_compare = val2str(compared_value);
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val.contains(val_compare.as_str()),
				}
			}
			// 关联
			CellValue::FieldTypeReference(vals) => {
				let val = match vals.get(0) {
					Some(val) => val.to_owned(),
					None => "".to_owned(),
				};
				let val_compares = match compared_value.as_array() {
					Some(v) => v
						.iter()
						.map(|v| {
							let v = v.get("text").unwrap();
							let v = v.as_str().unwrap();
							v.to_owned()
						})
						.collect::<Vec<_>>(),
					None => vec![],
				};
				let val_compare = match val_compares.get(0) {
					Some(val) => val.to_owned(),
					None => "".to_owned(),
				};
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val_compares.contains(&val),
				}
			}
			// 双向关联
			CellValue::FieldTypeTwoWayLinkRecords(vals) => {
				let val = match vals.get(0) {
					Some(val) => val.text.to_owned(),
					None => "".to_owned(),
				};
				let val_compares = val2arr(compared_value);
				let val_compare = match val_compares.get(0) {
					Some(val) => val.to_owned(),
					None => "".to_owned(),
				};
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val_compares.contains(&val),
				}
			}
			// 地理位置
			CellValue::FieldTypeLocation(val) => {
				let val = match val {
					Some(val) => val.title.to_string(),
					None => "".to_owned(),
				};
				let val_compare = val2str(compared_value);
				match op {
					CompareType::Eq => val == val_compare,
					CompareType::Neq => val != val_compare,
					CompareType::Gt => val > val_compare,
					CompareType::Gte => val >= val_compare,
					CompareType::Lt => val < val_compare,
					CompareType::Lte => val <= val_compare,
					CompareType::In => val.contains(val_compare.as_str()),
				}
			}
			// 公式
			// 货币
			// CellValue::FieldTypeCurrency(val) => val.to_string(),
			// 群
			// 自动编号
			CellValue::FieldTypeAutonumber(val) => {
				let val_compare = val2str(compared_value);
				match op {
					CompareType::Eq => val.text == val_compare,
					CompareType::Neq => val.text != val_compare,
					CompareType::Gt => val.text > val_compare,
					CompareType::Gte => val.text >= val_compare,
					CompareType::Lt => val.text < val_compare,
					CompareType::Lte => val.text <= val_compare,
					CompareType::In => val.text.contains(val_compare.as_str()),
				}
			}
			_ => false,
		};
		return val;
	}
}

#[allow(dead_code)]
impl super::super::index::WeixinWork {
	pub(crate) async fn record_compare<T, R>(
		&self,
		value1: &CellValue,
		value2: &CellValue,
		get_referenced_str: T,
	) -> bool
	where
		T: Fn(String) -> R,
		R: std::future::Future<Output = String>,
	{
		log::debug!("record_compare: {:#?}==={:#?}", value1, value2);
		let val = match value1 {
			// 文本
			CellValue::FieldTypeText(_val1) => {
				let v1 = self.record_val2str(value1);
				let v2 = self.record_val2str(value2);
				v1 == v2
			}
			// 数字
			CellValue::FieldTypeNumber(_val1) => {
				let v1 = self.record_val2f64(value1);
				let v2 = self.record_val2f64(value2);
				v1 == v2
			}
			// 复选框
			CellValue::FieldTypeCheckbox(val1) => match value2 {
				CellValue::FieldTypeCheckbox(val2) => val1 == val2,
				_ => false,
			},
			// 日期
			CellValue::FieldTypeDateTime(val1) => match value2 {
				CellValue::FieldTypeDateTime(val2) => val1 == val2,
				_ => false,
			},
			// 图片
			CellValue::FieldTypeImage(val1) => match value2 {
				CellValue::FieldTypeImage(val2) => {
					if val1.len() == val2.len() {
						val1.iter().enumerate().all(|(i, v1)| {
							let v2 = &val2[i];
							v1.title == v2.title
								&& v1.image_url == v2.image_url
								&& v1.height == v2.height
								&& v1.width == v2.width && v1.id == v2.id
						})
					} else {
						false
					}
				}
				_ => false,
			},
			// 文件
			CellValue::FieldTypeAttachment(val1) => match value2 {
				CellValue::FieldTypeAttachment(val2) => {
					if val1.len() == val2.len() {
						val1.iter().enumerate().all(|(i, v1)| {
							let v2 = &val2[i];
							v1.doc_type == v2.doc_type
								&& v1.file_ext == v2.file_ext
								&& v1.file_type == v2.file_type
								&& v1.file_url == v2.file_url
								&& v1.name == v2.name && v1.size == v2.size
						})
					} else {
						false
					}
				}
				_ => false,
			},
			// 成员
			CellValue::FieldTypeUser(val1) => match value2 {
				CellValue::FieldTypeUser(val2) => {
					if val1.len() == val2.len() {
						val1.iter().enumerate().all(|(i, v1)| {
							let v2 = &val2[i];
							v1.user_id == v2.user_id
								&& v1.tmp_external_userid == v2.tmp_external_userid
						})
					} else {
						false
					}
				}
				_ => false,
			},
			// 超链接
			CellValue::FieldTypeUrl(val1) => match value2 {
				CellValue::FieldTypeUrl(val2) => {
					if val1.len() == val2.len() {
						val1.iter().enumerate().all(|(i, v1)| {
							let v2 = &val2[i];
							v1.link == v2.link
								&& v1.text == v2.text && v1.value_type == v2.value_type
						})
					} else {
						false
					}
				}
				_ => false,
			},
			// 多选
			CellValue::FieldTypeSelect(val1) => match value2 {
				CellValue::FieldTypeSelect(val2) => {
					if val1.len() == val2.len() {
						val1.iter().enumerate().all(|(i, v1)| {
							let v2 = &val2[i];
							v1.id == v2.id && v1.text == v2.text && v1.style == v2.style
						})
					} else {
						false
					}
				}
				_ => false,
			},
			// 创建人
			CellValue::FieldTypeExtCreateName(val1) => {
				let v1 = val1.to_owned();
				let v2 = self.record_val2str(value2);
				v1 == v2
			}
			// 最后编辑人
			CellValue::FieldTypeExtUpdateName(val1) => {
				let v1 = val1.to_owned();
				let v2 = self.record_val2str(value2);
				v1 == v2
			}
			// 创建时间
			CellValue::FieldTypeExtCreateTime(val1) => {
				let v1 = *val1 as f64;
				let v2 = self.record_val2f64(value2);
				v1 == v2
			}
			// 最后编辑时间
			CellValue::FieldTypeExtUpdateTime(val1) => {
				let v1 = *val1 as f64;
				let v2 = self.record_val2f64(value2);
				v1 == v2
			}
			// 进度
			CellValue::FieldTypeProgress(val1) => match value2 {
				CellValue::FieldTypeProgress(val2) => val1 == val2,
				_ => false,
			},
			// 货币
			CellValue::FieldTypeCurrency(val1) => match value2 {
				CellValue::FieldTypeCurrency(val2) => val1 == val2,
				_ => false,
			},
			// 电话
			CellValue::FieldTypePhoneNumber(val1) => match value2 {
				CellValue::FieldTypePhoneNumber(val2) => val1 == val2,
				_ => false,
			},
			// 邮件
			CellValue::FieldTypeEmail(val1) => match value2 {
				CellValue::FieldTypeEmail(val2) => val1 == val2,
				_ => false,
			},
			// 单选
			CellValue::FieldTypeSingleSelect(val1) => match value2 {
				CellValue::FieldTypeSingleSelect(val2) => val1 == val2,
				_ => false,
			},
			// 关联
			CellValue::FieldTypeReference(val1) => {
				let v1 = match val1.first() {
					Some(v) => v,
					None => "",
				};
				// v1为recordid,根据行号获取记录数据
				let v1 = get_referenced_str(v1.to_owned()).await;
				let v2 = self.record_val2str(value2);
				// 因为不知道具体引用的是哪一个字段，这里只能比较字符串
				v1.contains(&v2)
			}
			// 双向关联
			CellValue::FieldTypeTwoWayLinkRecords(val1) => match value2 {
				CellValue::FieldTypeTwoWayLinkRecords(val2) => {
					if val1.len() == val2.len() {
						val1.iter().enumerate().all(|(i, v1)| {
							let v2 = &val2[i];
							v1.text == v2.text && v1.text_type == v2.text_type
						})
					} else {
						false
					}
				}
				_ => false,
			},
			// 地理位置
			CellValue::FieldTypeLocation(val1) => match value2 {
				CellValue::FieldTypeLocation(val2) => match val1 {
					None => val2.is_none(),
					Some(v1) => match val2 {
						Some(v2) => {
							v1.id == v2.id
								&& v1.latitude == v2.latitude
								&& v1.longitude == v2.longitude
								// && v1.source_type == v2.source_type
								&& v1.title == v2.title
						}
						None => false,
					},
				},
				_ => false,
			},
			// 公式
			// 货币
			// CellValue::FieldTypeCurrency(val) => val.to_string(),
			// 群
			// 自动编号
			CellValue::FieldTypeAutonumber(val1) => match value2 {
				CellValue::FieldTypeAutonumber(val2) => {
					val1.seq == val2.seq && val1.text == val2.text
				}
				_ => false,
			},
			_ => false,
		};
		return val;
	}
}
