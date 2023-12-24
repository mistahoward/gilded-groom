use crate::structs::api_response::ApiResponse;
use super::response::*;
use super::service::request_token;

#[get("/token?<code>&<error>")]
pub async fn token_test(code: Option<String>, error: Option<String>) -> ApiResponse<TokenResponse> {
	if code.is_none() && error.is_none() {
		return ApiResponse::bad_request("code or error is required");
	}
	if code.is_some() && error.is_some() {
		return ApiResponse::bad_request("code and error cannot be used together");
	}
	if error.is_some() {
		return ApiResponse::unauthorized(error.unwrap_or_default());
	}

	let token_response: Result<TokenResponse, reqwest::Error> = request_token(code.unwrap()).await;

	if token_response.is_err() {
		return ApiResponse::unauthorized(token_response.unwrap_err().to_string());
	}

	ApiResponse::ok(token_response.unwrap())
}

pub fn get_routes() -> Vec<rocket::Route> {
	routes![token_test]
}