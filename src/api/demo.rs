use serde_json::json;

#[actix_web::get("/")]
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

#[actix_web::post("/test/")]
pub async fn test_post(data: actix_web::web::Query<Param>) -> actix_web::HttpResponse {
	log::debug!("param = {}", data.data);
	return actix_web::HttpResponse::Ok().json(json!({
		"data": 100,
		"msg": "ok".to_owned(),
	}));
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Data {
	data: u32,
	msg: String,
}

#[actix_web::post("/test2/")]
pub async fn test_post2(data: actix_web::web::Json<Data>) -> actix_web::HttpResponse {
	log::debug!("param = {:#?}", data);
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
	return actix_web::HttpResponse::Ok().json(Data {
		data: 100,
		msg: data.msg.clone(),
	});
}

#[derive(Debug, serde::Deserialize)]
pub struct Query {
	data: u32,
	msg: String,
}

#[actix_web::get("/db")]
pub async fn db(
	_req: actix_web::HttpRequest,
	query: actix_web::web::Query<Query>,
	state: actix_web::web::Data<std::sync::Arc<crate::app_state::AppState>>,
) -> String {
	log::debug!("param = {:#?}", query);
	log::debug!("data = {}", query.data);
	log::debug!("msg = {}", query.msg);
	let client = &state.mssql;
	let rows = crate::db::mssql::sys_user::SysUser::all()
		.limit(3)
		.run(client)
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

#[actix_web::get("/db2")]
pub async fn db2(
	_req: actix_web::HttpRequest,
	state: actix_web::web::Data<std::sync::Arc<crate::app_state::AppState>>,
) -> actix_web::HttpResponse {
	type Table = crate::db::postgres::a::A;
	let client = &state.pg;
	let rows = Table::all().limit(3).run(client).await.unwrap();
	let row = rows.first().unwrap();
	dbg!(row);

	return actix_web::HttpResponse::Ok().json(serde_json::json!({
		"foo": "bar"
	}));
}

#[derive(Debug, serde::Serialize)]
struct ReturnResult {
	foo: i32,
	bar: i32,
}

#[actix_web::get("/db3")]
pub async fn db3(
	_req: actix_web::HttpRequest,
	state: actix_web::web::Data<std::sync::Arc<crate::app_state::AppState>>,
) -> actix_web::HttpResponse {
	let mut client = state.mssql().await;
	let stream = client
		.query("SELECT @P1 as foo, @P2 as bar", &[&1i32, &2i32])
		.await
		.unwrap();
	// let results = stream.into_results().await.unwrap();	// 多条查询语句时使用
	let mut results = stream.into_first_result().await.unwrap();
	let mut data: Vec<ReturnResult> = results
		.drain(..)
		.map(|r| {
			let foo = r.get(0).unwrap();
			let bar = r.get("bar").unwrap_or(0);
			ReturnResult { foo, bar }
		})
		.collect();
	let data = data.pop().unwrap();
	log::debug!("{:#?}", data);
	let mut client = state.mssql().await;
	client
		.execute(
			"INSERT INTO ##Test (id) VALUES (@P1), (@P2), (@P3)",
			&[&1i32, &2i32, &3i32],
		)
		.await
		.unwrap();
	return actix_web::HttpResponse::Ok().json(serde_json::json!({
		"foo": "bar"
	}));
}
