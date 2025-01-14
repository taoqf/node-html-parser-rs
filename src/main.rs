mod api;
mod app_state;
mod atoms;
mod db;

#[tokio::main(flavor = "multi_thread", worker_threads = 12)]
async fn main() {
	log4rs::init_file("./log4rs.yaml", Default::default()).expect("log4rs config not correct!");
	dotenvy::dotenv().unwrap();
	let shared_state = std::sync::Arc::new(app_state::AppState::new().await);
	let app = axum::Router::new()
		.nest(
			"/api",
			axum::Router::new()
				.route("/", axum::routing::get(api::demo::hello))
				.route("/test-post", axum::routing::post(api::demo::test_post))
				.route("/test-post2", axum::routing::post(api::demo::test_post2))
				.route("/db1", axum::routing::post(api::demo::db))
				.route("/db2", axum::routing::post(api::demo::db2)),
		)
		.with_state(shared_state);

	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
