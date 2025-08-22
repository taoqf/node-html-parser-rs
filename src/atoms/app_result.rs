pub(crate) fn json(
	value: serde_json::Value,
) -> actix_web::Result<actix_web::HttpResponse, crate::err::AppError> {
	return Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
		"ok": true,
		"data": value
	})));
}

#[allow(dead_code)]
pub(crate) fn text(
	value: &str,
) -> actix_web::Result<actix_web::HttpResponse, crate::err::AppError> {
	return Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
		"ok": true,
		"data": value
	})));
}

#[allow(dead_code)]
pub(crate) fn err(value: &str) -> actix_web::Result<actix_web::HttpResponse, crate::err::AppError> {
	return Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
		"ok": false,
		"msg": value
	})));
}
