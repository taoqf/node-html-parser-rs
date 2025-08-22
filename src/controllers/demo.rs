use anyhow::Context;
use welds::Client;

pub(crate) struct Controller;

impl Controller {
	pub(crate) async fn new() -> Self {
		Self
	}
}
lazy_static::lazy_static! {
	static ref CONTROLLER: tokio::sync::OnceCell<Controller> = tokio::sync::OnceCell::new();
}

#[allow(dead_code)]
pub(crate) async fn get_ctrl() -> &'static Controller {
	CONTROLLER
		.get_or_init(|| async { Controller::new().await })
		.await
}

impl Controller {
	#[allow(dead_code)]
	pub(crate) async fn hello(&self) -> anyhow::Result<&'static str> {
		let url = format!("http://127.0.0.1:3000/api/{}?data=123", "test");
		let client = reqwest::Client::new();

		#[derive(Debug, serde::Serialize, serde::Deserialize)]
		pub(crate) struct Reply {
			pub(crate) data: u32,
			pub(crate) msg: String,
		}

		let data = client
			.get(url)
			// .json(&json!({
			// 	"foo": "bar",
			// }))
			.send()
			.await?
			.json::<Reply>()
			.await?;

		log::debug!("{:?}", data);
		return Ok("Hello world!");
	}
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct TestPostParam {
	data: u32,
}

impl Controller {
	#[allow(dead_code)]
	pub(crate) async fn testpost(&self, data: TestPostParam) -> anyhow::Result<&'static str> {
		log::debug!("{:#?}", data);
		return Ok("Hello world!");
	}
}

impl Controller {
	#[allow(dead_code)]
	pub(crate) async fn db(&self) -> anyhow::Result<serde_json::Value> {
		type Table = crate::db::postgres::tb01sys::Tb01Sys;
		let state = crate::get_state().await;
		let client = state.pg.as_ref();
		let rows = Table::all().limit(3).run(client).await?;
		let row = rows.first().context("No row")?;
		dbg!(row);
		return Ok(serde_json::json!({
			"foo": "bar"
		}));
	}
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Db2Param {}

impl Controller {
	#[allow(dead_code)]
	pub(crate) async fn db2(&self, param: Db2Param) -> anyhow::Result<serde_json::Value> {
		log::debug!("db2/param {:#?}", param);
		#[derive(Debug, serde::Serialize)]
		struct ReturnResult {
			foo: i32,
			bar: i32,
		}

		let state = crate::get_state().await;
		let client = state.pg.as_ref();
		let mut rows = client.fetch_rows("SELECT 1 as foo, 2 as bar", &[]).await?;
		let mut data: Vec<ReturnResult> = rows
			.drain(..)
			.map(|r| {
				let foo = r.get_by_position(0).unwrap_or(0);
				let bar = r.get("bar").unwrap_or(0);
				ReturnResult { foo, bar }
			})
			.collect();
		let data = data.pop().context("Could not get data")?;
		log::debug!("{:#?}", data);
		return Ok(serde_json::json!(data));
	}
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Db3Param {}

impl Controller {
	#[allow(dead_code)]
	pub(crate) async fn db3(&self, param: Db3Param) -> anyhow::Result<serde_json::Value> {
		log::debug!("db3/param {:#?}", param);
		type Table = crate::db::mssql::sys_user::SysUser;
		let state = crate::get_state().await;
		let client = state.pg.as_ref(); // !!! should be mssql client
		let rows = Table::all().limit(3).run(client).await?;
		let row = rows.first().context("No row")?;
		dbg!(row);
		return Ok(serde_json::json!({
			"foo": "bar"
		}));
	}
}
