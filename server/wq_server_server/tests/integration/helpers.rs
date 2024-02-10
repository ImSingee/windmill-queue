use pavex::server::Server;
use serde::{Deserialize, Serialize};
use wq_server::app::db::sea_orm::ConnectionTrait;
use wq_server::app::db::Connection;
use wq_server::configuration::Config;
use wq_server_server::configuration::{load_configuration, ApplicationProfile};
use wq_server_server_sdk::{build_application_state, run};

pub struct TestApi {
    pub api_address: String,
    pub api_client: reqwest::Client,
    pub db: Connection,
}

impl TestApi {
    pub async fn spawn() -> Self {
        let mut config = Self::get_config();

        let (database_url, db) = test_db(&config.database.url).await;
        config.database.url = database_url;

        let application_state = build_application_state(config.clone()).await;

        let tcp_listener = config
            .server
            .listener()
            .await
            .expect("Failed to bind the server TCP listener");
        let address = tcp_listener
            .local_addr()
            .expect("The server TCP listener doesn't have a local socket address");
        let server_builder = Server::new().listen(tcp_listener);

        tokio::spawn(async move { run(server_builder, application_state).await });

        TestApi {
            api_address: format!("http://{}:{}", config.server.ip, address.port()),
            api_client: reqwest::Client::new(),
            db,
        }
    }

    fn get_config() -> Config {
        load_configuration(Some(ApplicationProfile::Test))
            .expect("Failed to load test configuration")
    }
}

async fn test_db(database_url: &str) -> (String, Connection) {
    let suffix = format!("_test_{}", uuid::Uuid::now_v7());
    let mut database_name = database_url.split("/").last().unwrap().to_string();
    database_name.push_str(&suffix);

    // Create database
    let conn = Connection::connect(&database_url).await;
    conn.execute_unprepared(&format!(r#"CREATE DATABASE "{}";"#, database_name))
        .await
        .expect("Failed to create database.");

    // Connect to created database
    let database_url = format!("{}{}", database_url, suffix);
    let connection = Connection::connect(&database_url).await;

    // Migrate the created database
    connection
        .migrate()
        .await
        .expect("Failed to migrate database.");

    (database_url, connection)
}

/// Convenient methods for calling the API under test.
impl TestApi {
    pub async fn get_ping(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/api/ping", &self.api_address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn ingest_events<T: Serialize>(&self, body: T) -> reqwest::Response {
        let body = serde_json::to_string(&body).unwrap();

        self.api_client
            .post(&format!("{}/api/ingest", &self.api_address))
            .header("Content-Type", "application/json; charset=utf-8")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub code: String,
    pub message: String,
}

pub trait ResponseExt {
    async fn to_error_response(self) -> ErrorResponse;
}

impl ResponseExt for reqwest::Response {
    async fn to_error_response(self) -> ErrorResponse {
        let response = self.json::<ErrorResponse>().await.unwrap();

        assert_eq!(response.success, false);

        response
    }
}
