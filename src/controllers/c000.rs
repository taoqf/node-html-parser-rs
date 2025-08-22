use anyhow::{Context, Ok};

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
pub(crate) struct A01Param {}

impl Controller {
	pub(crate) async fn a001(&self, param: A01Param) -> anyhow::Result<serde_json::Value> {
		log::debug!("c001/param {:#?}", param);
		return Ok(serde_json::json!({}));
	}
	pub(crate) async fn a002(&self) -> anyhow::Result<serde_json::Value> {
		log::debug!("c002");
		let num = "mm"
			.parse::<i32>()
			.context("Could not convert to interger")?;
		return Ok(serde_json::json!({
			"num":num
		}));
	}
}
