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
pub(crate) async fn get_c001() -> &'static Controller {
	CONTROLLER
		.get_or_init(|| async { Controller::new().await })
		.await
}

impl Controller {
	pub(crate) async fn getappid(&self) -> serde_json::Value {
		let state = crate::get_state().await;
		let appid = state.appid.as_str();
		return serde_json::json!({ "appid": appid });
	}
}
