use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};

pub type Event = JsonMap<String, JsonValue>;

#[derive(Debug, Deserialize, Serialize)]
pub struct EventWithMeta {
    pub queue: String,
    pub event: Event,
    pub meta: EventMeta,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventMeta {
    pub id: String,
    pub ts: u64,
}

