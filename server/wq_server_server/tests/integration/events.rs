use crate::helpers::{ResponseExt, TestApi};
use pavex::http::StatusCode;
use serde_json::json;
use wq_server::app::db::diesel::dsl::count_star;
use wq_server::app::db::diesel::prelude::*;
use wq_server::app::db::diesel_async::RunQueryDsl;
use wq_server::app::db::models;
use wq_server::app::db::schema::queue;

#[tokio::test]
async fn insert_events_works() {
    let api = TestApi::spawn().await;

    let response = api
        .ingest_events(json!({
            "events": [
                {
                    "event": {
                        "hello": 1
                    }
                },
                {
                    "event": {
                        "hello": 2
                    },
                    "meta": {
                        "id": "123",
                    }
                },
                {
                    "event": {
                        "id": "666",
                        "hello": 1
                    },
                    "meta": {
                        "idKey": "id",
                    }
                },
                {
                    "event": {
                        "hello": 1
                    },
                    "meta": {
                        "ts": 1707177603744u64
                    }
                },
            ],
            "meta": {
                "queue": "test",
            }
        }))
        .await;

    let status = response.status();

    assert_eq!(status.as_u16(), StatusCode::OK.as_u16());

    // wait 100ms for executor to process the events
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // check if events are inserted
    let mut conn = api.db.conn().await.expect("Failed to get connection");
    let count: i64 = queue::table
        .select(count_star())
        .first(&mut conn)
        .await
        .expect("Failed to count events");

    assert_eq!(count, 4);
}

#[tokio::test]
async fn insert_events_de_duplicate() {
    let api = TestApi::spawn().await;

    let response = api
        .ingest_events(json!({
            "events": [
                {
                    "event": {},
                    "meta": {
                        "id": "test1",
                        "ts": 1,
                    }
                },
                {
                    "event": {},
                    "meta": {
                         "id": "test1",
                         "ts": 2,
                    }
                },
                {
                    "event": {
                        "id": "test1"
                    },
                    "meta": {
                        "idKey": "id",
                        "ts": 3,
                    }
                },
            ],
            "meta": {
                "queue": "test",
            }
        }))
        .await;

    let status = response.status();

    assert_eq!(status.as_u16(), StatusCode::OK.as_u16());

    // wait 100ms for executor to process the events
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // TODO
    let mut conn = api.db.conn().await.expect("Failed to get connection");
    let all = models::Queue::all().load(&mut conn).await.unwrap();

    assert_eq!(all.len(), 1);

    let first = &all[0];

    assert_eq!(first.ts.timestamp_millis(), 1);
}

#[tokio::test]
async fn insert_events_fail() {
    let api = TestApi::spawn().await;

    let response = api
        .ingest_events(json!({
            "events": [
                {
                    "event": {
                        "id": "666",
                        "hello": 1
                    },
                    "meta": {
                        "idKey": "not-exist",
                    }
                }
            ],
            "meta": {
                "queue": "test",
            }
        }))
        .await;

    let status = response.status();

    assert_eq!(status.as_u16(), StatusCode::BAD_REQUEST.as_u16());

    let error_response = response.to_error_response().await;

    assert_eq!(error_response.code, "E40001");
}
