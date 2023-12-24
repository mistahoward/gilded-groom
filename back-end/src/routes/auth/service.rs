use std::env;
use serde_json;

use super::response::TokenResponse;

pub async fn request_token(auth_code: String) -> Result<TokenResponse, reqwest::Error> {
	let client_id: String = env::var("OAUTH_CLIENT_ID").expect("OAUTH_CLIENT_ID must be set");
	let client_secret: String = env::var("OAUTH_CLIENT_SECRET").expect("OAUTH_CLIENT_SECRET must be set");
	let redirect_uri: String = env::var("OAUTH_REDIRECT_URI").expect("OAUTH_REDIRECT_URI must be set");
	const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

	let request_body: serde_json::Value = serde_json::json!({
		"code": auth_code,
		"client_id": client_id,
		"client_secret": client_secret,
		"redirect_uri": redirect_uri,
		"grant_type": "authorization_code"
	});

	let response = reqwest::Client::new()
		.post(TOKEN_URL)
		.json(&request_body)
		.send()
		.await?
		.json()
		.await?;

	let token_response: TokenResponse = serde_json::from_value(response).unwrap();

	Ok(token_response)
}