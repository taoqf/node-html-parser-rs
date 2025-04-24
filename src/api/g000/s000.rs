#[actix_web_codegen::routes]
#[post("/s000/a000")]
#[post("/s000-a000")]
pub(crate) async fn a001(
	param: actix_web::web::Json<crate::ctrls::c000::A00Param>,
) -> crate::AppResponse {
	let ctrl = crate::ctrls::get_c000().await;
	let value = ctrl.a000(param.into_inner()).await?;
	return crate::atoms::app_result::json(value);
}
