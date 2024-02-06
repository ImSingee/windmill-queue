use apalis::prelude::Storage;
use pavex::request::body::JsonBody;
use pavex::response::Response;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;
use crate::app::event::{Event, EventWithMeta, EventMeta};
use crate::app::queue::{NewEvents, NewEventsProducer};
use crate::utils::pavex::json_response;
use crate::utils::error::{HTTPResult, HTTPError};
use crate::utils::error_code::ErrorCode;


#[derive(Deserialize, Debug)]
pub struct In {
    pub events: Vec<InEventWithMeta>,
    pub meta: Meta,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
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
    pub trace_id: Option<String>,
}

#[derive(Debug, Error)]
enum EventConvertError {
    #[error("idKey {0} does not exist in provided event")]
    IdKeyNotExist(String)
}

impl InEventWithMeta {
    fn into_event(self, queue: &str) -> Result<EventWithMeta, EventConvertError> {
        let id = if let Some(id) = self.meta.id {
            id
        } else if let Some(id_key) = self.meta.id_key {
            if let Some(id) = self.event.get(&id_key) {
                id.as_str().ok_or_else(|| EventConvertError::IdKeyNotExist(id_key.clone()))?.to_string()
            } else {
                return Err(EventConvertError::IdKeyNotExist(id_key));
            }
        } else {
            ulid::Ulid::new().to_string()
        };

        let trace_id = self.meta.trace_id.unwrap_or_else(|| id.to_string());
        let ts = self.meta.ts.unwrap_or_else(|| chrono::Utc::now().timestamp_millis() as u64);

        Ok(EventWithMeta {
            queue: queue.to_string(),
            event: self.event,
            meta: EventMeta {
                id,
                ts,
                trace_id,
            },
        })
    }
}


pub async fn ingest_events(JsonBody(In { meta, events }): JsonBody<In>, mut producer: NewEventsProducer) -> HTTPResult<Response> {
    let events = events.into_iter().map(|event| event.into_event(&meta.queue)).collect::<std::result::Result<Vec<_>, _>>();
    let events = match events {
        Ok(events) => events,
        Err(err) => {
            return Err(HTTPError::bad_request(ErrorCode::E40001, "invalid event", err));
        }
    };

    let result = producer.push(NewEvents { events_with_meta: events }).await;
    if let Err(err) = result {
        return Err(HTTPError::internal_server_error(ErrorCode::E50001, "failed to persist events", err));
    }

    Ok(json_response(json!({"success": true})))
}