#[allow(dead_code)]
pub(crate) struct AppState {
	pub appid: String,
	pub appsecret: String,
	pub pg: welds::connections::postgres::PostgresClient,
	pub mssql: welds::connections::mssql::MssqlClient,
}

impl AppState {
	pub(crate) async fn new() -> Self {
		let appid = std::env::var("WX_APPID").unwrap();
		log::debug!("appid={}", appid);
		let appsecret = std::env::var("WX_APPSECRET").unwrap();
		log::debug!("appsecret={}", appsecret);

		let pool_size: u32 = std::env::var("DB_POOL").unwrap().parse().unwrap();

		let pg = get_pg("DB_PG", pool_size).await;
		let mssql = get_mssql("DB_MSSQL", pool_size).await;
		return Self {
			appid,
			appsecret,
			pg,
			mssql,
		};
	}
}

async fn get_pg(env_key: &str, pool_size: u32) -> welds::connections::postgres::PostgresClient {
	let url_db = std::env::var(env_key).unwrap();
	log::debug!("DB_URL={}", url_db);
	let pool = sqlx::postgres::PgPoolOptions::new()
		.max_connections(pool_size)
		.connect(&url_db)
		.await
		.unwrap();
	let client: welds::connections::postgres::PostgresClient = pool.into();
	return client;
}

async fn get_mssql(env_key: &str, pool_size: u32) -> welds::connections::mssql::MssqlClient {
	let url_db = std::env::var(env_key).unwrap();
	log::debug!("DB_URL={}", url_db);
	let mgr = bb8_tiberius::ConnectionManager::build(url_db.as_str()).unwrap();
	let pool = bb8::Pool::builder()
		.max_size(pool_size)
		.build(mgr)
		.await
		.unwrap();
	let client: welds::connections::mssql::MssqlClient = pool.into();
	return client;
}

async fn get_mysql(env_key: &str, _pool_size: u32) -> welds::connections::mysql::MysqlClient {
	let url_db = std::env::var(env_key).unwrap();
	log::debug!("DB_URL={}", url_db);
	let pool = sqlx::MySqlPool::connect(&url_db).await.unwrap();
	let client: welds::connections::mysql::MysqlClient = pool.into();
	return client;
}
