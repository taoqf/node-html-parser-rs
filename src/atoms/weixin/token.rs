#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(super) struct TokenSaved {
	pub(super) token: String,
	pub(super) expired: u64,
}

impl TokenSaved {
	fn new(token: &TokenGot) -> Self {
		let now = crate::atoms::dt::now_stamp();
		let expired = now + token.expires_in * 1000 - 6000; // 这里，减去6000，提前一分钟更新token
		let token = token.access_token.clone();
		return Self {
			token: token.to_owned(),
			expired,
		};
	}
}

pub(super) async fn get_access_token(
	appid: &str,
	appsecret: &str,
	url: &str,
	key: &str,
	db: &dyn welds::Client,
) -> TokenSaved {
	assert!(appid.is_empty() == false, "appid must be set.");
	assert!(appsecret.is_empty() == false, "appsecret must be set.");
	let client = db;

	let now = crate::atoms::dt::now_stamp();

	let data = crate::db::postgres::tb01sys::Tb01Sys::all()
		.where_col(|r| r.key.equal(key))
		.limit(1)
		.run(client)
		.await
		.unwrap();
	let token = if data.is_empty() {
		let token = get_access_token_from_server(appid, appsecret, url).await;
		let token = TokenSaved::new(&token);
		// save token to db
		let token_saved = serde_json::to_string(&token).unwrap();

		let mut data = crate::db::postgres::tb01sys::Tb01Sys::new();
		data.key = key.to_owned();
		data.value = Some(token_saved);
		data.save(client).await.unwrap();
		token
	} else {
		let row = data.first().unwrap();
		let token_str = row.value.clone().unwrap_or_default();
		let token = serde_json::from_str::<TokenSaved>(&token_str).unwrap();
		if token.token.is_empty() || token.expired < now {
			let token = get_access_token_from_server(appid, appsecret, url).await;
			let token = TokenSaved::new(&token);
			// save token to db
			let token_saved = serde_json::to_string(&token).unwrap();
			crate::db::postgres::tb01sys::Tb01Sys::where_col(|r| r.key.equal(key))
				.set(|r| r.value, token_saved)
				.run(client)
				.await
				.unwrap();
			token
		} else {
			token
		}
	};

	return token;
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TokenGot {
	access_token: String,
	expires_in: u64,
	errcode: i32,
	errmsg: String,
}

async fn get_access_token_from_server(appid: &str, appsecret: &str, url: &str) -> TokenGot {
	assert!(appid.is_empty() == false, "appid must be set.");
	assert!(appsecret.is_empty() == false, "appsecret must be set.");

	return reqwest::get(url)
		.await
		.unwrap()
		.json::<TokenGot>()
		.await
		.unwrap();
}
