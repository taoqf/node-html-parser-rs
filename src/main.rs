pub(crate) mod api;
pub(crate) mod app_state;
pub(crate) mod atoms;
pub(crate) mod controllers;
pub(crate) mod db;
pub(crate) mod err;

pub(crate) use app_state::get_state;
#[allow(unused_imports)]
pub(crate) use controllers as ctrls;
pub(crate) use err::AppResponse;

#[actix_web::main]
async fn main() {
	log4rs::init_file("./log4rs.yaml", Default::default()).expect("log4rs config not correct!");
	dotenvy::dotenv().unwrap();
	let state = crate::get_state().await;
	actix_web::HttpServer::new(move || {
		actix_web::App::new()
			.wrap(actix_web::middleware::Logger::default())
			.service(
				actix_web::web::scope(&state.appid)
					.service(actix_web::web::scope("g000").service(api::g000::s000::a001)),
			)
	})
	// .workers(12)
	.bind(("0.0.0.0", 3000))
	.unwrap()
	.run()
	.await
	.unwrap();
}
