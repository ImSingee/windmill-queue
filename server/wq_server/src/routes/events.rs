use pavex::http::StatusCode;
use pavex::request::body::JsonBody;
use pavex::request::path::PathParams;
use pavex::response::Response;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;
use crate::app::event::{AnyMessage, Event, EventMeta};
use crate::utils::pavex::{json_response, json_response_with_status};

#[PathParams]
pub struct Params {
    pub queue: String,
}

#[derive(Deserialize, Debug)]
pub struct InEvent {
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

#[derive(Debug, Error)]
enum EventConvertError {
    #[error("idKey {0} does not exist in provided event")]
    IdKeyNotExist(String)
}

impl InEvent {
    fn into_event(self, queue: &str) -> Result<Event, EventConvertError> {
        let ts = self.meta.ts.unwrap_or_else(|| chrono::Utc::now().timestamp_millis() as u64);

        let id = if let Some(id) = self.meta.id {
            id
        } else if let Some(id_key) = self.meta.id_key {
            if let Some(id) = self.message.get(&id_key) {
                id.as_str().ok_or_else(|| EventConvertError::IdKeyNotExist(id_key.clone()))?.to_string()
            } else {
                return Err(EventConvertError::IdKeyNotExist(id_key));
            }
        } else {
            Uuid::now_v7().to_string()
        };

        Ok(Event {
            queue: queue.to_string(),
            message: self.message,
            meta: EventMeta {
                id,
                ts,
            },
        })
    }
}

pub async fn ingest_events(PathParams(Params { queue }): &PathParams<Params>, JsonBody(events): JsonBody<Vec<InEvent>>) -> Response {
    let events = events.into_iter().map(|event| event.into_event(&queue)).collect::<Result<Vec<_>, _>>();
    let events = match events {
        Ok(events) => events,
        Err(err) => {
            return json_response_with_status(StatusCode::BAD_REQUEST, json!({"success": false, "error": err.to_string()}));
        }
    };

    for event in events {
        println!("Ingest Event: {:?}", event);
    }

    json_response(json!({"success": true}))
}