use actix_web::{self, web, App, HttpServer};

#[derive(Clone)]
pub(crate) struct AppState {
	pool_size: u32,
	url_db_pg: String,
	url_db_mssql: String,
}

impl AppState {
	pub(crate) fn new() -> Self {
		let pool_size: u32 = std::env::var("DB_POOL").unwrap().parse().unwrap();
		let url_db_pg = std::env::var("DB_PG").unwrap();
		log::debug!("url_db_pg={}", url_db_pg);
		let url_db_mssql = std::env::var("DB_MSSQL").unwrap();
		log::debug!("url_db_mssql={}", url_db_mssql);
		return Self {
			pool_size,
			url_db_mssql,
			url_db_pg,
		};
	}
}
impl AppState {
	pub(crate) async fn pg(&self) -> welds::connections::postgres::PostgresClient {
		let url_db = &self.url_db_pg;
		let pool = sqlx::postgres::PgPoolOptions::new()
			.max_connections(self.pool_size)
			.connect(url_db)
			.await
			.unwrap();
		let client: welds::connections::postgres::PostgresClient = pool.into();
		return client;
	}
}
impl AppState {
	pub(crate) async fn mssql(&self) -> welds::connections::mssql::MssqlClient {
		let url_db = &self.url_db_mssql;
		let mgr = bb8_tiberius::ConnectionManager::build(url_db.as_str()).unwrap();
		let pool = bb8::Pool::builder()
			.max_size(self.pool_size)
			.build(mgr)
			.await
			.unwrap();
		let client: welds::connections::mssql::MssqlClient = pool.into();
		return client;
	}
}

mod api;
mod atoms;
mod db;

// #[tokio::main]
// async fn main() {
fn main() {
	log4rs::init_file("./log4rs.yaml", Default::default()).expect("log4rs config not correct!");
	dotenvy::dotenv().unwrap();
	start_server().unwrap();
}

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(AppState::new()))
			.service(
				web::scope("/api")
					.service(api::demo::hello)
					.service(api::demo::test_post)
					.service(api::demo::db)
					.service(api::demo::db2),
			)
	})
	// .workers(12)
	.bind(("0.0.0.0", 3000))?
	.run()
	.await
}
