#[allow(dead_code)]
pub(crate) struct AppState {
	pub appid: String,
	pub appsecret: String,
	pub pg: welds::connections::postgres::PostgresClient,
	// pub mssql: welds::connections::mssql::MssqlClient,
	// pub mysql: welds::connections::mysql::MysqlClient,
	// mssql2: deadpool_tiberius::Pool,
}

impl AppState {
	pub(crate) async fn new() -> Self {
		let appid = std::env::var("WX_APPID").unwrap();
		log::debug!("appid={}", appid);
		let appsecret = std::env::var("WX_APPSECRET").unwrap();
		log::debug!("appsecret={}", appsecret);

		let pg = get_pg("DB_PG").await;
		// let mssql = get_mssql("DB_MSSQL").await;
		// let mysql = get_mysql("DB_MYSQL").await;
		return Self {
			appid,
			appsecret,
			pg,
			// mssql,
			// mysql,
		};
	}
}

async fn get_pg(env_key: &str) -> welds::connections::postgres::PostgresClient {
	let url_db = std::env::var(env_key).unwrap();
	log::debug!("DB_URL={}", url_db);
	let client = welds::connections::postgres::connect(url_db.as_str())
		.await
		.unwrap();
	return client;
}

#[allow(dead_code)]
async fn get_mssql(env_key: &str) -> welds::connections::mssql::MssqlClient {
	let url_db = std::env::var(env_key).unwrap();
	log::debug!("DB_URL={}", url_db);
	let client = welds::connections::mssql::connect(url_db.as_str())
		.await
		.unwrap();
	return client;
}

#[allow(dead_code)]
async fn get_mssql2(env_key: &str, pool_size: u32) -> deadpool_tiberius::Pool {
	let url_db = std::env::var(env_key).unwrap();
	log::debug!("DB_URL={}", url_db);

	let pool = deadpool_tiberius::Manager::from_ado_string(&url_db)
		.unwrap()
		.max_size(pool_size as usize)
		.create_pool()
		.unwrap();
	return pool;
}

#[allow(dead_code)]
async fn get_mysql(env_key: &str) -> welds::connections::mysql::MysqlClient {
	let url_db = std::env::var(env_key).unwrap();
	log::debug!("DB_URL={}", url_db);
	let client = welds::connections::mysql::connect(url_db.as_str())
		.await
		.unwrap();
	return client;
}
