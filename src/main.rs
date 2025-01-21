mod api;
mod app_state;
mod atoms;
mod db;

#[actix_web::main]
async fn main() {
	log4rs::init_file("./log4rs.yaml", Default::default()).expect("log4rs config not correct!");
	dotenvy::dotenv().unwrap();
	let shared_state = std::sync::Arc::new(app_state::AppState::new().await);
	actix_web::HttpServer::new(move || {
		actix_web::App::new()
			.app_data(actix_web::web::Data::new(shared_state.clone()))
			.service(
				actix_web::web::scope("/api")
					.service(api::demo::hello)
					.service(api::demo::test_post)
					.service(api::demo::db)
					.service(api::demo::db2),
			)
	})
	// .workers(12)
	.bind(("0.0.0.0", 3000))
	.unwrap()
	.run()
	.await
	.unwrap();
}
