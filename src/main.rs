pub(crate) mod api;
pub(crate) mod app_state;
pub(crate) mod atoms;
pub(crate) mod controllers;
pub(crate) mod db;
pub(crate) mod err;

pub(crate) use app_state::get_state;
#[allow(unused_imports)]
pub(crate) use controllers as ctrls;
pub(crate) use err::AppResponse;

#[actix_web::main]
async fn main() {
	log4rs::init_file("./log4rs.yaml", Default::default()).expect("log4rs config not correct!");
	dotenvy::dotenv().unwrap();
	let state = crate::get_state().await;
	actix_web::HttpServer::new(move || {
		actix_web::App::new()
			.wrap(actix_web::middleware::Logger::default())
			.service(
				actix_web::web::scope(&state.appid)
					.wrap(actix_web::middleware::from_fn(auth_middleware))
					.service(
						actix_web::web::scope("g000")
							.service(api::g000::s000::a000)
							.service(api::g000::s000::a001)
							.service(api::g000::s000::a002),
					),
			)
	})
	// .workers(12)
	.bind(("0.0.0.0", 3000))
	.unwrap()
	.run()
	.await
	.unwrap();
}

async fn auth_middleware(
	req: actix_web::dev::ServiceRequest,
	next: actix_web::middleware::Next<impl actix_web::body::MessageBody>,
) -> Result<actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>, actix_web::Error> {
	let uri = req.uri().to_string();
	log::debug!("Calling {}", uri);
	let headers = req.headers();
	// 用户鉴权
	if let Some(auth_header) = headers.get("Authorization") {
		// 模拟异步用户鉴权
		if auth_header == "valid-token" {
			// 如果鉴权成功，继续处理请求
			return next.call(req).await;
			// let res = next.call(req).await;
			// return match res {
			// 	Ok(res) => {
			// 		log::debug!("Success calling {}", uri);
			// 		Ok(res)
			// 	}
			// 	Err(err) => {
			// 		// 调用服务失败
			// 		log::error!("Runtime Error while calling {}:{}", uri, err);
			// 		Err(err)
			// 	}
			// };
		}
	}

	return Err(actix_web::Error::from(err::AppError::msg("Unauthorized")));
}
