use crate::app::queue::{NewEvents, NewEventsProducer};
use crate::app::queue_events::{Event, EventMeta, EventWithMeta, EventsMeta};
use crate::utils::error::{HTTPError, HTTPResult};
use crate::utils::error_code::ErrorCode;
use crate::utils::pavex::json_response;
use apalis::prelude::Storage;
use pavex::request::body::JsonBody;
use pavex::response::Response;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub struct IngestRequest {
    pub events: Vec<IngestRequestEventWithMeta>,
    pub meta: EventsMeta,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct IngestResponseSuccess {
    /// Whether the request was successful (always true)
    pub success: bool,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct IngestRequestEventWithMeta {
    pub event: Event,
    #[serde(default = "Default::default")]
    pub meta: IngestRequestEventMeta,
}

#[derive(Deserialize, ToSchema, Default, Debug)]
pub struct IngestRequestEventMeta {
    pub id: Option<String>,
    #[serde(rename = "idKey")]
    pub id_key: Option<String>,
    pub ts: Option<u64>,
    pub trace_id: Option<String>,
}

#[derive(Debug, Error)]
pub enum EventConvertError {
    #[error("idKey {0} does not exist in provided event")]
    IdKeyNotExist(String),
}

impl TryFrom<IngestRequestEventWithMeta> for EventWithMeta {
    type Error = EventConvertError;

    fn try_from(value: IngestRequestEventWithMeta) -> Result<Self, Self::Error> {
        let id = if let Some(id) = value.meta.id {
            id
        } else if let Some(id_key) = value.meta.id_key {
            if let Some(id) = value.event.get(&id_key) {
                id.as_str()
                    .ok_or_else(|| EventConvertError::IdKeyNotExist(id_key.clone()))?
                    .to_string()
            } else {
                return Err(EventConvertError::IdKeyNotExist(id_key));
            }
        } else {
            ulid::Ulid::new().to_string()
        };

        let trace_id = value.meta.trace_id.unwrap_or_else(|| id.to_string());
        let ts = value
            .meta
            .ts
            .unwrap_or_else(|| chrono::Utc::now().timestamp_millis() as u64);

        Ok(EventWithMeta {
            event: value.event,
            meta: EventMeta { id, ts, trace_id },
        })
    }
}

#[utoipa::path(
    post,
    path = "/api/ingest",
    request_body = IngestRequest,
    responses(
        (status = 200, description = "success", body = IngestResponseSuccess),
        (status = 400, description = "E40001 Ingested Event is invalid")
    ),
)]
pub async fn ingest_events(
    JsonBody(IngestRequest { meta, events }): JsonBody<IngestRequest>,
    mut producer: NewEventsProducer,
) -> HTTPResult<Response> {
    let events = events
        .into_iter()
        .map(|event| event.try_into())
        .collect::<std::result::Result<Vec<EventWithMeta>, _>>();
    let events = match events {
        Ok(events) => events,
        Err(err) => {
            return Err(HTTPError::bad_request(
                ErrorCode::E40001,
                "invalid event",
                err,
            ));
        }
    };

    let result = producer.push(NewEvents { events, meta }).await;
    if let Err(err) = result {
        return Err(HTTPError::internal_server_error(
            ErrorCode::E50001,
            "failed to persist events",
            err,
        ));
    }

    Ok(json_response(json!({"success": true})))
}
