use pavex::request::body::JsonBody;
use pavex::request::path::PathParams;
use pavex::response::Response;
use serde::Deserialize;
use serde_json::json;
use crate::app::event::AnyMessage;
use crate::utils::pavex::json_response;

#[PathParams]
pub struct Params {
    pub queue: String,
}

#[derive(Deserialize, Debug)]
pub struct InEvents {
    pub message: AnyMessage,
    #[serde(default = "Default::default")]
    pub meta: InMeta,
}

#[derive(Deserialize, Default, Debug)]
pub struct InMeta {
    pub id: Option<String>,
    #[serde(rename = "idKey")]
    pub id_key: Option<String>,
    pub ts: Option<u64>,
}


pub async fn ingest_events(PathParams(Params { queue }): &PathParams<Params>, body: JsonBody<Vec<InEvents>>) -> Response {
    println!("Ingesting events into queue: {}", queue);
    for event in body.0.iter() {
        println!("Event: {:?}", event);
    }

    json_response(json!({"success": true}))
}