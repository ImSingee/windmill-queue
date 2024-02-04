use serde_json::{Map as JsonMap, Value as JsonValue};

pub type AnyMessage = JsonMap<String, JsonValue>;

#[derive(Debug)]
pub struct Event {
    pub queue: String,
    pub message: AnyMessage,
    pub meta: EventMeta,
}

#[derive(Debug)]
pub struct EventMeta {
    pub id: String,
    pub ts: u64,
}

