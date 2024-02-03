use pavex::response::Response;
use serde_json::json;
use crate::utils::pavex::json_response;

pub mod status;

pub fn root() -> Response {
    json_response(json!({"success": true}))
}