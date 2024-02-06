pub use crate::app::event::Event;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Deserialize, Serialize)]
pub struct EventsMeta {
    pub queue: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventWithMeta {
    pub event: Event,
    pub meta: EventMeta,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventMeta {
    pub id: String,
    pub ts: u64,
    pub trace_id: String,
}
