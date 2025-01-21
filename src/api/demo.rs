use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
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

#[derive(serde::Deserialize, serde::Serialize)]
struct Data {
	data: u32,
	msg: String,
}

#[post("/")]
pub async fn test_post(param: web::Json<Param>) -> HttpResponse {
	log::debug!("param = {}", param.data);
	HttpResponse::Ok().json(Data {
		data: 100,
		msg: "ok".to_owned(),
	})
}

#[tokio::test]
async fn test_post_api() {
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
}

#[derive(serde::Serialize)]
struct MyError {
	name: String,
}

#[derive(serde::Deserialize)]
struct Info {
	name: String,
}

#[derive(serde::Deserialize)]
struct Query {
	_name: String,
}

#[get("/db")]
pub async fn db(
	_req: actix_web::HttpRequest,
	_query: actix_web::web::Query<Query>,
	state: actix_web::web::Data<std::sync::Arc<crate::app_state::AppState>>,
) -> HttpResponse {
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
	return actix_web::HttpResponse::InternalServerError().body(msg);
}

#[get("/db2")]
pub async fn db2(
	_req: HttpRequest,
	state: actix_web::web::Data<std::sync::Arc<crate::app_state::AppState>>,
) -> HttpResponse {
	type Table = crate::db::postgres::a::A;
	let client = &state.pg;
	let rows = Table::all().limit(3).run(client).await.unwrap();
	let row = rows.first().unwrap();
	dbg!(row);
	return HttpResponse::Ok().json(MyError {
		name: "db2test".to_string(),
	});
}

#[derive(Debug, serde::Serialize)]
struct ReturnResult {
	foo: i32,
	bar: i32,
}

#[get("/db3")]
pub async fn db3(
	_req: HttpRequest,
	state: actix_web::web::Data<std::sync::Arc<crate::app_state::AppState>>,
) -> HttpResponse {
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
	return HttpResponse::Ok().json(MyError {
		name: "db3test".to_string(),
	});
}
