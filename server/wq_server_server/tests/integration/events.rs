use crate::helpers::{ResponseExt, TestApi};
use pavex::http::StatusCode;
use serde_json::json;
use wq_server::app::db::sea_orm::DatabaseBackend::Postgres;
use wq_server::app::db::sea_orm::{ConnectionTrait, Statement};

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
    let count = api
        .db
        .query_one(Statement::from_string(
            Postgres,
            "SELECT COUNT(*) FROM events",
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get_by_index::<i64>(0)
        .unwrap();
    assert_eq!(count, 4);
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
