use pavex::request::body::JsonBody;
use pavex::response::Response;
use serde::Deserialize;
use serde_json::json;
use crate::app::{Task, TaskSender};
use crate::utils::pavex::json_response;

#[derive(Deserialize)]
pub struct DemoTaskPayload {
    msg: String,
}

pub async fn new_demo_task(JsonBody(payload): JsonBody<DemoTaskPayload>, task_sender: TaskSender) -> Response {
    let msg = payload.msg;

    task_sender.send(Task::Demo(msg)).await.unwrap();

    json_response(json!({"success": true}))
}


