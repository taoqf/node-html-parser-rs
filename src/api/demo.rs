// use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder, Result};

// #[get("/")]
// pub async fn hello() -> impl Responder {
// 	HttpResponse::Ok().body("Hello world!")
// }

// #[derive(serde::Deserialize)]
// struct Param {
// 	data: u32,
// }

// #[post("/")]
// pub async fn test_post(param: web::Json<Param>) -> Result<String> {
// 	log::debug!("param = {}", param.data);
// 	Ok(format!("Welcome {}!", param.data))
// }

// #[derive(serde::Serialize)]
// struct MyError {
// 	name: String,
// }

// #[derive(serde::Deserialize)]
// struct Info {
// 	name: String,
// }

// #[get("")]
// pub async fn index(param: web::Query<Info>) -> impl Responder {
// 	log::debug!("name = {}", param.name);
// 	HttpResponse::Ok().json(MyError {
// 		name: param.name.clone(),
// 	})
// 	// HttpResponse::Ok().json(web::Json(MyError {
// 	// 	name: "abc".to_string(),
// 	// }))
// }

// #[get("/db")]
// pub async fn db(
// req: actix_web::HttpRequest,
// query: actix_web::web::Query<Query>,
// data: actix_web::web::Data<crate::AppState>,
// 	data: web::Data<crate::AppState>,
// ) -> HttpResponse {
// 	let conn = data.mssql().await;
// 	let rows = crate::db::mssql::sys_user::SysUser::all()
// 		.limit(3)
// 		.run(&conn)
// 		.await
// 		.unwrap();
// 	let row = rows.first().unwrap();
// 	let dt = row.modifydate.unwrap().format("%Y-%m-%d %H:%M:%S");
// 	let ab = row.modifydate.unwrap();
// 	let t = ab.and_utc().timestamp();
// 	dbg!(t);
// 	let msg = dt.to_string();
// 	dbg!(msg);
// 	return actix_web::HttpResponse::InternalServerError().body(msg);
// }

// #[get("/db2")]
// pub async fn db2(
// 	_req: HttpRequest,
// 	data: web::Data<crate::AppState>,
// ) -> Result<HttpResponse, Error> {
// 	let conn = data.pg().await;
// 	let rows = crate::db::postgres::a::A::all()
// 		.limit(3)
// 		.run(&conn)
// 		.await
// 		.unwrap();
// 	let row = rows.first().unwrap();
// 	dbg!(row);
// 	return HttpResponse::Ok().json(MyError {
// 		name: "db2test".to_string(),
// 	});
// }
