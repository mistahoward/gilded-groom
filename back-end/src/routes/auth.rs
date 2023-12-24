use crate::structs::api_response::ApiResponse;

#[get("/token?<code>&<error>")]
pub fn token_test(code: Option<String>, error: Option<String>) -> ApiResponse<&'static str> {
	if code.is_none() && error.is_none() {
		return ApiResponse::bad_request("code or error is required");
	}

	ApiResponse::ok("hello world")
}
