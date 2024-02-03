use pavex::http::StatusCode;
use pavex::response::body::Json;
use pavex::response::Response;
use serde::Serialize;

pub fn json_response_with_status<T: Serialize>(status: StatusCode, data: T) -> Response {
    Response::new(status).set_typed_body(Json::new(data).unwrap())
}

pub fn json_response<T: Serialize>(data: T) -> Response {
    json_response_with_status(StatusCode::OK, data)
}