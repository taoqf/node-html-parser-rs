pub(crate) type AppResponse = actix_web::Result<actix_web::HttpResponse, AppError>;

#[derive(Debug)]
pub(crate) struct AppError(anyhow::Error);

impl std::fmt::Display for AppError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl actix_web::error::ResponseError for AppError {
	fn error_response(&self) -> actix_web::HttpResponse {
		actix_web::HttpResponse::build(actix_web::http::StatusCode::SERVICE_UNAVAILABLE).json(
			serde_json::json!({
				"ok": false,
				"message": self.to_string()
			}),
		)
	}

	// fn status_code(&self) -> actix_web::http::StatusCode {
	// 	actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
	// }
}

impl From<anyhow::Error> for AppError {
	fn from(err: anyhow::Error) -> Self {
		return Self(err);
	}
}
