#[actix_web::get("/")]
pub(crate) async fn hello() -> &'static str {
	let url = format!("http://127.0.0.1:3000/api/{}?data=123", "test");
	let client = reqwest::Client::new();

	#[derive(Debug, serde::Serialize, serde::Deserialize)]
	pub(crate) struct Reply {
		pub(crate) data: u32,
		pub(crate) msg: String,
	}

	let data = client
		.get(url)
		// .json(&json!({
		// 	"foo": "bar",
		// }))
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
struct Param {
	data: u32,
}

#[actix_web::get("/test")]
pub(crate) async fn test_post(data: actix_web::web::Query<Param>) -> actix_web::HttpResponse {
	log::debug!("param = {}", data.data);
	let ctrlr = crate::controllers::get_c001().await;
	let value = ctrlr.getappid().await;
	return actix_web::HttpResponse::Ok().json(value);
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Data {
	data: u32,
	msg: String,
}

#[actix_web::post("/test2/")]
pub(crate) async fn test_post2(data: actix_web::web::Json<Data>) -> actix_web::HttpResponse {
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

// #[derive(Debug, serde::Deserialize)]
// struct Query {
// 	data: u32,
// 	msg: String,
// }

// #[actix_web::get("/db")]
// pub(crate) async fn db(
// 	_req: actix_web::HttpRequest,
// 	query: actix_web::web::Query<Query>,
// ) -> String {
// 	log::debug!("param = {:#?}", query);
// 	log::debug!("data = {}", query.data);
// 	log::debug!("msg = {}", query.msg);
// 	let state = crate::app_state::get_state().await;
// 	let client = state.mssql.as_ref();
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

// #[actix_web::get("/db2")]
// pub(crate) async fn db2(
// 	_req: actix_web::HttpRequest,
// ) -> actix_web::HttpResponse {
// 	type Table = crate::db::postgres::tb01sys::Tb01Sys;
// 	let client = state.pg.as_ref();
// 	let rows = Table::all().limit(3).run(client).await.unwrap();
// 	let row = rows.first().unwrap();
// 	dbg!(row);

// 	return actix_web::HttpResponse::Ok().json(serde_json::json!({
// 		"foo": "bar"
// 	}));
// }

// #[derive(Debug, serde::Serialize)]
// struct ReturnResult {
// 	foo: i32,
// 	bar: i32,
// }

// #[actix_web::get("/db3")]
// pub(crate) async fn db3(
// 	_req: actix_web::HttpRequest,
// ) -> actix_web::HttpResponse {
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
// 	return actix_web::HttpResponse::Ok().json(serde_json::json!({
// 		"foo": "bar"
// 	}));
// }
