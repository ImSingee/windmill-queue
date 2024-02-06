use crate::configuration::Config;
use sea_orm::DatabaseBackend::Postgres;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};
use std::ops::Deref;
use std::sync::Arc;

pub struct Connection(DatabaseConnection);

impl Deref for Connection {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Connection {
    pub async fn connect(database_url: &str) -> Connection {
        let mut opt = ConnectOptions::new(database_url);
        opt.sqlx_logging(false);

        let db = Database::connect(opt)
            .await
            .expect("Failed to connect to the database");

        Connection(db)
    }

    pub async fn new_pavex(config: Arc<Config>) -> Connection {
        let database_url = &config.database.url;

        let connection = Self::connect(database_url).await;

        let version = connection
            .query_one(Statement::from_string(Postgres, "SELECT version()"))
            .await
            .expect("cannot get pg version (exec sql fail)")
            .expect("cannot get pg version (no result)")
            .try_get_by_index::<String>(0)
            .expect("cannot get pg version (no column)");

        tracing::info!("Connected to the database ({version})");

        connection
    }
}
