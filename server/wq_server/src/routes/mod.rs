use crate::utils::pavex::json_response;
use pavex::response::Response;
use serde_json::json;

pub mod events;
pub mod openapi;
pub mod status;

pub fn root() -> Response {
    json_response(json!({"success": true}))
}
