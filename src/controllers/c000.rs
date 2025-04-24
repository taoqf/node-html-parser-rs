use anyhow::Ok;

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

#[derive(Debug, serde::Deserialize)]
pub(crate) struct A00Param {}

impl Controller {
	pub(crate) async fn a000(&self, param: A00Param) -> anyhow::Result<serde_json::Value> {
		log::debug!("c000/param {:#?}", param);
		return Ok(serde_json::json!({}));
	}
}
