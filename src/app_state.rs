#[allow(dead_code)]
pub(crate) struct AppState {
	pub(crate) appid: String,
	pub(crate) appsecret: String,
	// pub(crate) weixinwork: super::atoms::weixin::work::index::WeixinWork,
	// pub(crate) weixin: super::atoms::weixin::weixin::index::Weixin,
	// pub(crate) pg: Box<dyn welds::Client>,
	pub(crate) file_msg_encode_enable: bool,
	pub(crate) file_server: String,
	pub(crate) file_msg_encode_appid: String,
	pub(crate) file_msg_encode_safe_key: String,
}

lazy_static::lazy_static! {
	static ref STATE: tokio::sync::OnceCell<AppState> = tokio::sync::OnceCell::new();
}

#[allow(dead_code)]
pub(crate) async fn get_state() -> &'static AppState {
	STATE.get_or_init(|| async { AppState::new().await }).await
}

impl AppState {
	async fn new() -> Self {
		let appid = std::env::var("WX_APPID").unwrap();
		log::debug!("appid={}", appid);
		let appsecret = std::env::var("WX_APPSECRET").unwrap();
		log::debug!("appsecret={}", appsecret);

		// let pg = crate::atoms::db::get_db("DB_PG").await;
		let file_msg_encode_enable = std::env::var("FILE_MSG_ENCODE_ENABLE")
			.unwrap()
			.parse::<i32>()
			.unwrap();
		log::debug!("file_msg_encode_enable={}", file_msg_encode_enable);
		let file_server = std::env::var("FILE_SERVER").unwrap();
		log::debug!("file_server={}", file_server);
		let file_msg_encode_appid = std::env::var("FILE_MSG_ENCODE_APPID").unwrap();
		log::debug!("file_msg_encode_appid={}", file_msg_encode_appid);
		let file_msg_encode_safe_key = std::env::var("FILE_MSG_ENCODE_SAFE_KEY").unwrap();
		log::debug!("file_msg_encode_safe_key={}", file_msg_encode_safe_key);

		// let weixinwork = crate::atoms::weixin::work::index::WeixinWork::new().await;
		// let weixin = crate::atoms::weixin::weixin::index::Weixin::new().await;

		return Self {
			appid,
			appsecret,
			// weixin,
			// weixinwork,
			// pg,
			file_msg_encode_enable: file_msg_encode_enable == 1,
			file_server,
			file_msg_encode_appid,
			file_msg_encode_safe_key,
		};
	}
}

// #[allow(dead_code)]
// async fn get_pg(env_key: &str) -> welds::connections::postgres::PostgresClient {
// 	let url_db = std::env::var(env_key).unwrap();
// 	log::debug!("DB_URL={}", url_db);
// 	let client = welds::connections::postgres::connect(url_db.as_str())
// 		.await
// 		.unwrap();
// 	return client;
// }

// #[allow(dead_code)]
// async fn get_mssql(env_key: &str) -> welds::connections::mssql::MssqlClient {
// 	let url_db = std::env::var(env_key).unwrap();
// 	log::debug!("DB_URL={}", url_db);
// 	let client = welds::connections::mssql::connect(url_db.as_str())
// 		.await
// 		.unwrap();
// 	return client;
// }

// #[allow(dead_code)]
// async fn get_mssql2(env_key: &str, pool_size: u32) -> deadpool_tiberius::Pool {
// 	let url_db = std::env::var(env_key).unwrap();
// 	log::debug!("DB_URL={}", url_db);

// 	let pool = deadpool_tiberius::Manager::from_ado_string(&url_db)
// 		.unwrap()
// 		.max_size(pool_size as usize)
// 		.create_pool()
// 		.unwrap();
// 	return pool;
// }

// #[allow(dead_code)]
// async fn get_mysql(env_key: &str) -> welds::connections::mysql::MysqlClient {
// 	let url_db = std::env::var(env_key).unwrap();
// 	log::debug!("DB_URL={}", url_db);
// 	let client = welds::connections::mysql::connect(url_db.as_str())
// 		.await
// 		.unwrap();
// 	return client;
// }
