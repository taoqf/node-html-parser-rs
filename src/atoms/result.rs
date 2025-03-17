#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Result<T: serde::Serialize> {
	pub(crate) ok: bool,
	message: Option<String>,
	data: Option<T>,
}

impl<T> Result<T>
where
	T: serde::Serialize,
{
	#[allow(dead_code)]
	pub(crate) fn message(&self) -> &str {
		assert!(self.ok == false);
		assert!(self.message.is_some());
		return self.message.as_ref().unwrap();
	}
	#[allow(dead_code)]
	pub(crate) fn data(&self) -> &T {
		assert!(self.ok == true);
		assert!(self.data.is_some());
		return self.data.as_ref().unwrap();
	}
}

impl<T> Result<T>
where
	T: serde::Serialize,
{
	#[allow(dead_code)]
	pub(crate) fn ok(data: T) -> Self {
		log::debug!("Ok: {}", serde_json::to_string(&data).unwrap_or_default());
		Self {
			ok: true,
			message: None,
			data: Some(data),
		}
	}
}

impl Result<()> {
	#[allow(dead_code)]
	pub(crate) fn err(msg: &str) -> Self {
		log::error!("Err: {}", msg);
		Self {
			ok: false,
			message: Some(msg.to_owned()),
			data: None,
		}
	}
}

impl<T> Result<T>
where
	T: serde::Serialize,
{
	#[allow(dead_code)]
	pub(crate) fn res_ok(data: T) -> actix_web::HttpResponse {
		log::debug!(
			"Response Ok: {}",
			serde_json::to_string(&data).unwrap_or_default()
		);
		actix_web::HttpResponse::Ok().json(&Self {
			ok: true,
			message: None,
			data: Some(data),
		})
	}
}

impl Result<()> {
	#[allow(dead_code)]
	pub(crate) fn res_err(msg: &str) -> actix_web::HttpResponse {
		log::error!("Response Err: {}", msg);
		actix_web::HttpResponse::Ok().json(&Self {
			ok: false,
			message: Some(msg.to_owned()),
			data: None,
		})
	}
}
