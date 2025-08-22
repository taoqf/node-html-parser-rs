#[actix_web_codegen::routes]
#[post("/s000/a001")]
#[post("/s000-a001")]
pub(crate) async fn a001(
	param: actix_web::web::Json<crate::ctrls::c000::A01Param>,
) -> crate::AppResponse {
	let ctrl = crate::ctrls::get_c000().await;
	let value = ctrl.a001(param.into_inner()).await?;
	return crate::atoms::app_result::json(value);
}

#[actix_web_codegen::routes]
#[get("/s000/a002")]
#[get("/s000-a002")]
pub(crate) async fn a002() -> crate::AppResponse {
	let ctrl = crate::ctrls::get_c000().await;
	let value = ctrl.a002().await?;
	return crate::atoms::app_result::json(value);
	// Err(crate::err::AppError::msg("errmsg"))
}
