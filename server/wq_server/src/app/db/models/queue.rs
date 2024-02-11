use super::super::error::*;
use super::super::schema::*;
use chrono::NaiveDateTime;
use diesel::pg::Pg;
use diesel::prelude::*;
use serde_json::Value as JsonValue;
use uuid::Uuid;

pub type All = diesel::dsl::Select<queue::table, diesel::dsl::AsSelect<Queue, Pg>>;

#[derive(Debug, Queryable, Selectable, Identifiable, QueryableByName, AsChangeset)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = queue, check_for_backend(Pg))]
pub struct Queue {
    pub uuid: Uuid,
    pub ref_queue: String,
    pub ref_id: String,
    pub trace_id: String,
    pub event: JsonValue,
    pub meta: JsonValue,
    pub ts: NaiveDateTime,
    pub locked_by: Option<String>,
    pub locked_at: Option<NaiveDateTime>,
    pub scheduled_for: NaiveDateTime,
    pub retry_times: i32,
    pub logs: Vec<Option<JsonValue>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Queue {
    pub fn all() -> All {
        queue::table.select(Self::as_select())
    }
}

#[derive(Debug, Default, Insertable)]
#[diesel(table_name = queue, check_for_backend(Pg))]
pub struct NewQueue {
    pub uuid: Uuid,
    pub ref_queue: String,
    pub ref_id: String,
    pub trace_id: String,
    pub event: JsonValue,
    pub meta: JsonValue,
    pub ts: NaiveDateTime,
}
