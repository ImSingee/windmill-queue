// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    queue (uuid) {
        uuid -> Uuid,
        ref_queue -> Varchar,
        ref_id -> Varchar,
        trace_id -> Varchar,
        event -> Json,
        meta -> Json,
        ts -> Timestamp,
        locked_by -> Nullable<Varchar>,
        locked_at -> Nullable<Timestamp>,
        scheduled_for -> Timestamp,
        retry_times -> Int4,
        logs -> Array<Nullable<Json>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    queue,
);
