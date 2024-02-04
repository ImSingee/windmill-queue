use apalis::prelude::Storage;
use pavex::http::StatusCode;
use pavex::request::body::JsonBody;
use pavex::request::path::PathParams;
use pavex::response::Response;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;
use crate::app::event::{Event, EventWithMeta, EventMeta};
use crate::app::queue::{NewEvents, NewEventsProducer};
use crate::utils::pavex::{json_response, json_response_with_status};

#[PathParams]
pub struct Params {
    pub queue: String,
}

#[derive(Deserialize, Debug)]
pub struct InEventWithMeta {
    pub event: Event,
    #[serde(default = "Default::default")]
    pub meta: InEventMeta,
}

#[derive(Deserialize, Default, Debug)]
pub struct InEventMeta {
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

impl InEventWithMeta {
    fn into_event(self, queue: &str) -> Result<EventWithMeta, EventConvertError> {
        let ts = self.meta.ts.unwrap_or_else(|| chrono::Utc::now().timestamp_millis() as u64);

        let id = if let Some(id) = self.meta.id {
            id
        } else if let Some(id_key) = self.meta.id_key {
            if let Some(id) = self.event.get(&id_key) {
                id.as_str().ok_or_else(|| EventConvertError::IdKeyNotExist(id_key.clone()))?.to_string()
            } else {
                return Err(EventConvertError::IdKeyNotExist(id_key));
            }
        } else {
            Uuid::now_v7().to_string()
        };

        Ok(EventWithMeta {
            queue: queue.to_string(),
            event: self.event,
            meta: EventMeta {
                id,
                ts,
            },
        })
    }
}

pub async fn ingest_events(PathParams(Params { queue }): &PathParams<Params>, JsonBody(events): JsonBody<Vec<InEventWithMeta>>, mut producer: NewEventsProducer) -> Response {
    let events = events.into_iter().map(|event| event.into_event(&queue)).collect::<Result<Vec<_>, _>>();
    let events = match events {
        Ok(events) => events,
        Err(err) => {
            return json_response_with_status(StatusCode::BAD_REQUEST, json!({"success": false, "error": err.to_string()}));
        }
    };

    let result = producer.push(NewEvents { events_with_meta: events }).await;
    if let Err(err) = result {
        return json_response_with_status(StatusCode::INTERNAL_SERVER_ERROR, json!({"success": false, "error": err.to_string()}));
    }

    json_response(json!({"success": true}))
}