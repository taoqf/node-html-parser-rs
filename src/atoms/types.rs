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
#[derive(Debug, serde::Serialize)]
pub(crate) struct Result {
	ok: bool,
	message: Option<String>,
	data: Option<serde_json::Value>,
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
		actix_web::HttpResponse::Ok().json(Self {
			ok: true,
			message: None,
			data: Some(data),
		})
	}
	#[allow(dead_code)]
	pub(crate) fn res_err(msg: &str) -> actix_web::HttpResponse {
		actix_web::HttpResponse::Ok().json(Self {
			ok: false,
			message: Some(msg.to_owned()),
			data: None,
		})
	}
}
