use serde_json::{Map as JsonMap, Value as JsonValue};

pub type AnyMessage = JsonMap<String, JsonValue>;

pub struct Event {
    pub queue: String,
    pub message: AnyMessage,
    pub meta: EventMeta,
}

pub struct EventMeta {
    pub id: String,
    pub ts: u64,
}

