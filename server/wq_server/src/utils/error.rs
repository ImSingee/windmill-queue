use crate::utils::error_code::ErrorCode;
use crate::utils::pavex::json_response_with_status;
use pavex::http::StatusCode;
use pavex::response::{IntoResponse, Response};
use serde_json::json;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

pub type HTTPResult<T> = std::result::Result<T, HTTPError>;

#[derive(Debug, Error)]
pub struct HTTPError {
    status: StatusCode,
    code: ErrorCode,
    message: String,
    #[source]
    error: anyhow::Error,
}

impl Display for HTTPError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "An error occurred: code={} message={}",
            self.code, self.message
        )
    }
}

impl HTTPError {
    pub fn new(
        status: StatusCode,
        code: ErrorCode,
        message: impl Into<String>,
        error: impl Into<anyhow::Error>,
    ) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            error: error.into(),
        }
    }

    pub fn bad_request(
        code: ErrorCode,
        message: impl Into<String>,
        error: impl Into<anyhow::Error>,
    ) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code, message, error)
    }

    pub fn internal_server_error(
        code: ErrorCode,
        message: impl Into<String>,
        error: impl Into<anyhow::Error>,
    ) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, code, message, error)
    }
}

impl IntoResponse for &HTTPError {
    fn into_response(self) -> Response {
        let HTTPError {
            status,
            code,
            message,
            error,
        } = self;

        let body = json!({
            "success": false,
            "code": code.to_string(),
            "message": message,
            "error": format!("{}", error),
        });

        json_response_with_status(status.clone(), body)
    }
}

pub fn error_handler(error: &HTTPError) -> Response {
    error.into_response()
}
