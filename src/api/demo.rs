use serde_json::json;

#[ntex::web::get("/actix")]
pub async fn hello() -> &'static str {
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

#[ntex::web::get("/test")]
pub async fn test_post(data: ntex::web::types::Query<Param>) -> ntex::web::HttpResponse {
	log::debug!("param = {}", data.data);
	return ntex::web::HttpResponse::Ok().json(&json!({
		"data": 100,
		"msg": "ok".to_owned(),
	}));
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Data {
	data: u32,
	msg: String,
}

#[ntex::web::post("/test2/")]
pub async fn test_post2(data: ntex::web::types::Json<Data>) -> ntex::web::HttpResponse {
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
	return ntex::web::HttpResponse::Ok().json(&Data {
		data: 100,
		msg: data.msg.clone(),
	});
}

// #[derive(Debug, serde::Deserialize)]
// pub struct Query {
// 	data: u32,
// 	msg: String,
// }

// #[ntex::web::get("/db")]
// pub async fn db(
// 	_req: ntex::web::HttpRequest,
// 	query: ntex::web::types::Query<Query>,
// 	state: ntex::web::types::State<std::sync::Arc<crate::app_state::AppState>>,
// ) -> String {
// 	log::debug!("param = {:#?}", query);
// 	log::debug!("data = {}", query.data);
// 	log::debug!("msg = {}", query.msg);
// 	let client = &state.mssql;
// 	let rows = crate::db::mssql::sys_user::SysUser::all()
// 		.limit(3)
// 		.run(client)
// 		.await
// 		.unwrap();
// 	let row = rows.first().unwrap();
// 	let dt = row.modifydate.unwrap().format("%Y-%m-%d %H:%M:%S");
// 	let ab = row.modifydate.unwrap();
// 	let t = ab.and_utc().timestamp();
// 	dbg!(t);
// 	let msg = dt.to_string();
// 	return msg;
// }

#[ntex::web::get("/db2")]
pub async fn db2(
	_req: ntex::web::HttpRequest,
	state: ntex::web::types::State<std::sync::Arc<crate::app_state::AppState>>,
) -> ntex::web::HttpResponse {
	type Table = crate::db::postgres::a::A;
	let client = &state.pg;
	let rows = Table::all().limit(3).run(client).await.unwrap();
	let row = rows.first().unwrap();
	dbg!(row);

	return ntex::web::HttpResponse::Ok().json(&serde_json::json!({
		"foo": "bar"
	}));
}

// #[derive(Debug, serde::Serialize)]
// struct ReturnResult {
// 	foo: i32,
// 	bar: i32,
// }

// #[ntex::web::get("/db3")]
// pub async fn db3(
// 	_req: ntex::web::HttpRequest,
// 	state: ntex::web::types::State<std::sync::Arc<crate::app_state::AppState>>,
// ) -> ntex::web::HttpResponse {
// 	let mut client = state.mssql().await;
// 	let stream = client
// 		.query("SELECT @P1 as foo, @P2 as bar", &[&1i32, &2i32])
// 		.await
// 		.unwrap();
// 	// let results = stream.into_results().await.unwrap();	// 多条查询语句时使用
// 	let mut results = stream.into_first_result().await.unwrap();
// 	let mut data: Vec<ReturnResult> = results
// 		.drain(..)
// 		.map(|r| {
// 			let foo = r.get(0).unwrap();
// 			let bar = r.get("bar").unwrap_or(0);
// 			ReturnResult { foo, bar }
// 		})
// 		.collect();
// 	let data = data.pop().unwrap();
// 	log::debug!("{:#?}", data);
// 	let mut client = state.mssql().await;
// 	client
// 		.execute(
// 			"INSERT INTO ##Test (id) VALUES (@P1), (@P2), (@P3)",
// 			&[&1i32, &2i32, &3i32],
// 		)
// 		.await
// 		.unwrap();
// 	return ntex::web::HttpResponse::Ok().json(&serde_json::json!({
// 		"foo": "bar"
// 	}));
// }
