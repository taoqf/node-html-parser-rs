mod api;
mod app_state;
mod atoms;
mod db;

#[ntex::main]
async fn main() {
	log4rs::init_file("./log4rs.yaml", Default::default()).expect("log4rs config not correct!");
	dotenvy::dotenv().unwrap();
	// actix_web::web::scope("/api")
	// 	.service(api::demo::hello)
	// 	.service(api::demo::test_post),
	let shared_state = std::sync::Arc::new(app_state::AppState::new().await);
	ntex::web::HttpServer::new(move || {
		ntex::web::App::new().state(shared_state.clone()).service(
			ntex::web::scope("/api")
				.service(api::demo::hello)
				.service(api::demo::test_post),
		)
	})
	.bind(("0.0.0.0", 3000))
	.unwrap()
	.run()
	.await
	.unwrap();
}
