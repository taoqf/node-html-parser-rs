use serde_json::json;

pub async fn hello() -> &'static str {
	let url = format!("http://127.0.0.1:3000/api/{}", "erp");
	let client = reqwest::Client::new();

	#[derive(Debug, serde::Serialize, serde::Deserialize)]
	pub(crate) struct Reply {
		#[serde(rename = "RequestStatus")]
		pub(crate) request_status: String,
		#[serde(rename = "RequestStatusCode")]
		pub(crate) request_status_code: u32,
		#[serde(rename = "ReplyMode")]
		pub(crate) reply_mode: u32, // ReplyModeEnum
		// pub(crate) reply_mode: ReplyModeEnum,
		#[serde(rename = "ReplyAlertMessage")]
		pub(crate) reply_alert_message: String,
		#[serde(rename = "ReplyContent")]
		pub(crate) reply_content: String,
		pub(crate) error: String,
		#[serde(rename = "errorCode")]
		pub(crate) error_code: u32,
		#[serde(rename = "errorMessage")]
		pub(crate) error_message: String,
		pub(crate) msgcode: u32,
		pub(crate) message: String,
	}

	let data = client
		.post(url)
		.json(&json!({
			"foo": "bar",
		}))
		.send()
		.await
		.unwrap()
		.json::<Reply>()
		.await
		.unwrap();

	log::debug!("{:?}", data);
	return "Hello world!";
}

#[tokio::test]
async fn test_hello_api() {
	let res = reqwest::get("http://127.0.0.1:3000/api/")
		.await
		.unwrap()
		.text()
		.await
		.unwrap();
	assert_eq!(res, "Hello world!");
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Param {
	data: u32,
}

pub async fn test_post(
	axum::extract::Json(data): axum::extract::Json<Param>,
) -> axum::response::Json<serde_json::Value> {
	log::debug!("param = {}", data.data);
	return axum::response::Json(json!({
		"data": 100,
		"msg": "ok".to_owned(),
	}));
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Data {
	data: u32,
	msg: String,
}

pub async fn test_post2(
	axum::extract::Json(data): axum::extract::Json<serde_json::Value>,
) -> axum::response::Json<Data> {
	log::debug!("param = {}", data);
	let client = reqwest::Client::new();
	let res = client
		.post("http://127.0.0.1:3000/api/")
		.json(&Param { data: 10 })
		.send()
		.await
		.unwrap()
		.json::<Data>()
		.await
		.unwrap();
	assert_eq!(res.data, 100);
	assert_eq!(res.msg, "ok");
	return axum::response::Json(Data {
		data: 100,
		msg: data.get("foo").unwrap().to_string(),
	});
}

pub async fn db(
	axum::extract::State(data): axum::extract::State<std::sync::Arc<crate::app_state::AppState>>,
) -> String {
	let conn = &data.mssql;
	let rows = crate::db::mssql::sys_user::SysUser::all()
		.limit(3)
		.run(conn)
		.await
		.unwrap();
	let row = rows.first().unwrap();
	let dt = row.modifydate.unwrap().format("%Y-%m-%d %H:%M:%S");
	let ab = row.modifydate.unwrap();
	let t = ab.and_utc().timestamp();
	dbg!(t);
	let msg = dt.to_string();
	return msg;
}

#[derive(serde::Deserialize)]
pub struct Query {
	name: String,
}

pub async fn db2(
	axum::extract::State(data): axum::extract::State<std::sync::Arc<crate::app_state::AppState>>,
	axum::extract::Query(query): axum::extract::Query<Query>,
) -> String {
	type Table = crate::db::postgres::a::A;
	let client = &data.pg;
	let rows = Table::all().limit(3).run(client).await.unwrap();
	let row = rows.first().unwrap();
	dbg!(row);
	return query.name;
}
