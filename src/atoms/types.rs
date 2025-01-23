#[allow(dead_code)]
pub(crate) type TIMESTAMP = Vec<u8>;
#[allow(dead_code)]
pub(crate) type DATETIME = chrono::NaiveDateTime;
#[allow(dead_code)]
pub(crate) type DECIMAL = tiberius::numeric::Decimal;
#[allow(dead_code)]
pub(crate) type DECIMALL = tiberius::numeric::Decimal;
#[allow(dead_code)]
pub(crate) type FLOAT = f64;
#[allow(dead_code)]
pub(crate) type NCHAR = String;
#[allow(dead_code)]
pub(crate) type CHAR = String;
#[allow(dead_code)]
pub(crate) type MONEY = f64;
#[allow(dead_code)]
pub(crate) type IMAGE = Vec<u8>;
#[allow(dead_code)]
pub(crate) type Uuid = String;
#[allow(dead_code)]
pub(crate) type SYSNAME = String;
#[allow(dead_code)]
pub(crate) type NUMERIC = tiberius::numeric::Decimal;
#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Result {
	pub ok: bool,
	message: Option<String>,
	data: Option<serde_json::Value>,
}

impl Result {
	#[allow(dead_code)]
	pub(crate) fn message(&self) -> &str {
		assert!(self.ok == false);
		assert!(self.message.is_some());
		return self.message.as_ref().unwrap();
	}
	#[allow(dead_code)]
	pub(crate) fn data(&self) -> &serde_json::Value {
		assert!(self.ok == true);
		assert!(self.data.is_some());
		return self.data.as_ref().unwrap();
	}
}

impl Result {
	#[allow(dead_code)]
	pub(crate) fn ok(data: serde_json::Value) -> Self {
		Self {
			ok: true,
			message: None,
			data: Some(data),
		}
	}
	#[allow(dead_code)]
	pub(crate) fn err(msg: &str) -> Self {
		Self {
			ok: false,
			message: Some(msg.to_owned()),
			data: None,
		}
	}
}

impl Result {
	#[allow(dead_code)]
	pub(crate) fn res_ok(data: serde_json::Value) -> actix_web::HttpResponse {
		actix_web::HttpResponse::Ok().json(&Self {
			ok: true,
			message: None,
			data: Some(data),
		})
	}
	#[allow(dead_code)]
	pub(crate) fn res_err(msg: &str) -> actix_web::HttpResponse {
		actix_web::HttpResponse::Ok().json(&Self {
			ok: false,
			message: Some(msg.to_owned()),
			data: None,
		})
	}
}
