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
		actix_web::HttpResponse::Ok().json(&Self {
			ok: false,
			message: Some(msg.to_owned()),
			data: None,
		})
	}
}
