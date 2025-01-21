#[allow(dead_code)]
pub(crate) struct AppState {
	pub appid: String,
	pub appsecret: String,
	pub pg: welds::connections::postgres::PostgresClient,
	pub mssql: welds::connections::mssql::MssqlClient,
	pub mysql: welds::connections::mysql::MysqlClient,
	mssql2: deadpool_tiberius::Pool,
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
		let mysql = get_mysql("DB_MYSQL", pool_size).await;
		let mssql2 = get_mssql2("DB_MSSQL2", pool_size).await;
		return Self {
			appid,
			appsecret,
			pg,
			mssql,
			mysql,
			mssql2,
		};
	}
}

impl AppState {
	pub(crate) async fn mssql(
		&self,
	) -> deadpool_tiberius::deadpool::managed::Object<deadpool_tiberius::Manager> {
		let conn = self.mssql2.get().await.unwrap();
		return conn;
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

async fn get_mysql(env_key: &str, _pool_size: u32) -> welds::connections::mysql::MysqlClient {
	let url_db = std::env::var(env_key).unwrap();
	log::debug!("DB_URL={}", url_db);
	let pool = sqlx::MySqlPool::connect(&url_db).await.unwrap();
	let client: welds::connections::mysql::MysqlClient = pool.into();
	return client;
}
