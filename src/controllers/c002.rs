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
pub(crate) async fn get_c002() -> &'static Controller {
	CONTROLLER
		.get_or_init(|| async { Controller::new().await })
		.await
}
