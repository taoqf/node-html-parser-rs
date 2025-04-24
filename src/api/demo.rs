#[actix_web::get("/")]
pub(crate) async fn hello() -> crate::AppResponse {
	let ctrl = crate::ctrls::get_demo().await;
	let value = ctrl.hello().await?;
	return crate::atoms::app_result::text(value);
}

#[actix_web::get("/test")]
pub(crate) async fn test_post(
	param: actix_web::web::Query<crate::ctrls::demo::TestPostParam>,
) -> crate::AppResponse {
	let ctrl = crate::ctrls::get_demo().await;
	let value = ctrl.testpost(param.into_inner()).await?;
	return crate::atoms::app_result::text(value);
}

#[actix_web::get("/db")]
pub(crate) async fn db(_req: actix_web::HttpRequest) -> crate::AppResponse {
	let ctrl = crate::ctrls::get_demo().await;
	let val = ctrl.db().await?;
	return crate::atoms::app_result::json(val);
}

#[actix_web::get("/db2")]
pub(crate) async fn db2(
	param: actix_web::web::Json<crate::ctrls::demo::Db2Param>,
) -> crate::AppResponse {
	let ctrl = crate::ctrls::get_demo().await;
	let val = ctrl.db2(param.into_inner()).await?;
	return crate::atoms::app_result::json(val);
}
