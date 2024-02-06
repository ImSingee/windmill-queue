use crate::utils::error_code::ErrorCode;
use crate::utils::pavex::json_response_with_status;
use pavex::http::StatusCode;
use pavex::response::{IntoResponse, Response};
use serde_json::json;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

pub type HTTPResult<T> = std::result::Result<T, HTTPError>;

type BoxedError = Box<dyn std::error::Error + 'static + Send + Sync>;

#[derive(Debug, Error)]
pub struct HTTPError {
    status: StatusCode,
    code: ErrorCode,
    message: String,
    #[source]
    source: Option<BoxedError>,
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

#[derive(Debug)]
pub enum PhantomError {}

impl Display for PhantomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "phantom error")
    }
}

impl std::error::Error for PhantomError {}

unsafe impl Send for PhantomError {}
unsafe impl Sync for PhantomError {}

impl HTTPError {
    pub fn new<E: std::error::Error + 'static + Send + Sync>(
        status: StatusCode,
        code: ErrorCode,
        message: impl Into<String>,
        source: Option<E>,
    ) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            source: source.map(|e| Box::new(e) as BoxedError),
        }
    }

    pub fn bad_request<E: std::error::Error + 'static + Send + Sync>(
        code: ErrorCode,
        message: impl Into<String>,
        source: Option<E>,
    ) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code, message, source)
    }

    pub fn internal_server_error<E: std::error::Error + 'static + Send + Sync>(
        code: ErrorCode,
        message: impl Into<String>,
        source: Option<E>,
    ) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, code, message, source)
    }
}

impl IntoResponse for &HTTPError {
    fn into_response(self) -> Response {
        let HTTPError {
            status,
            code,
            message,
            source,
        } = self;

        let body = json!({
            "success": false,
            "code": code.to_string(),
            "message": message,
        });

        json_response_with_status(status.clone(), body)
    }
}

pub fn error_handler(error: &HTTPError) -> Response {
    error.into_response()
}
