use pavex::http::StatusCode;
use serde_json::json;
use crate::helpers::{ResponseExt, TestApi};

#[tokio::test]
async fn insert_events_works() {
    let api = TestApi::spawn().await;

    let response = api.ingest_events(json!({
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
    })).await;

    let status = response.status();

    assert_eq!(status.as_u16(), StatusCode::OK.as_u16());
}

#[tokio::test]
async fn insert_events_fail() {
    let api = TestApi::spawn().await;

    let response = api.ingest_events(json!({
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
    })).await;

    let status = response.status();

    assert_eq!(status.as_u16(), StatusCode::BAD_REQUEST.as_u16());

    let error_response = response.to_error_response().await;

    assert_eq!(error_response.code, "E40001");
}
