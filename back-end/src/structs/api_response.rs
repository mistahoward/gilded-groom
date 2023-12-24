use rocket::serde::json::Json;
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::{Responder, self};
use std::fmt::Debug;

#[derive(Debug)]
pub enum ErrorWrapper {
	Unauthorized(String),
	BadRequest(String),
	InternalServerError(String),
	NotFound(String),
	Conflict(String),
}

impl ErrorWrapper {
	pub fn status(&self) -> Status {
		match self {
			ErrorWrapper::Unauthorized(_) => Status::Unauthorized,
			ErrorWrapper::BadRequest(_) => Status::BadRequest,
			ErrorWrapper::InternalServerError(_) => Status::InternalServerError,
			ErrorWrapper::NotFound(_) => Status::NotFound,
			ErrorWrapper::Conflict(_) => Status::Conflict,
		}
	}
	pub fn message(&self) -> String {
		match self {
			ErrorWrapper::Unauthorized(msg) => msg.to_string(),
			ErrorWrapper::BadRequest(msg) => msg.to_string(),
			ErrorWrapper::InternalServerError(msg) => msg.to_string(),
			ErrorWrapper::NotFound(msg) => msg.to_string(),
			ErrorWrapper::Conflict(msg) => msg.to_string(),
		}
	}
	pub fn unauthorized(msg: impl Into<String>) -> Self {
        ErrorWrapper::Unauthorized(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        ErrorWrapper::BadRequest(msg.into())
    }

    pub fn internal_server_error(msg: impl Into<String>) -> Self {
        ErrorWrapper::InternalServerError(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        ErrorWrapper::NotFound(msg.into())
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        ErrorWrapper::Conflict(msg.into())
    }
}

impl<'r> Responder<'r, 'static> for ErrorWrapper {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.message().respond_to(req)?)
            .status(self.status())
            .ok()
    }
}

#[derive(Debug, Responder)]
pub enum ApiResponse<T> {
	Ok(Json<T>),
	Err(ErrorWrapper),
}

impl<T> ApiResponse<T> {
	pub fn unauthorized(msg: impl Into<String>) -> Self {
		ApiResponse::Err(ErrorWrapper::Unauthorized(msg.into()))
	}

	pub fn bad_request(msg: impl Into<String>) -> Self {
		ApiResponse::Err(ErrorWrapper::BadRequest(msg.into()))
	}

	pub fn internal_server_error(msg: impl Into<String>) -> Self {
		ApiResponse::Err(ErrorWrapper::InternalServerError(msg.into()))
	}

	pub fn not_found(msg: impl Into<String>) -> Self {
		ApiResponse::Err(ErrorWrapper::NotFound(msg.into()))
	}

	pub fn conflict(msg: impl Into<String>) -> Self {
		ApiResponse::Err(ErrorWrapper::Conflict(msg.into()))
	}

	pub fn ok(data: T) -> Self {
		ApiResponse::Ok(Json(data))
	}
}