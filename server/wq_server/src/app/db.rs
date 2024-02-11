mod error;
mod migrate;
pub mod models;
pub mod schema;

use crate::configuration::Config;
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use migrate::run_migrations;
use std::sync::Arc;

pub use diesel;
pub use diesel_async;

pub use error::*;

#[derive(Clone)]
pub struct DB {
    pool: Pool<AsyncPgConnection>,
}

impl DB {
    pub fn connect(database_url: &str) -> Self {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(config)
            .build()
            .expect("Cannot connect to database");

        DB { pool }
    }

    pub async fn conn(&self) -> Result<Object<AsyncPgConnection>> {
        let conn = self.pool.get().await?;

        Ok(conn)
    }

    pub async fn migrate(database_url: &str) -> Result<()> {
        run_migrations(database_url).await
    }

    pub async fn new_pavex(config: Arc<Config>) -> Self {
        let database_url = &config.database.url;

        let connection = Self::connect(database_url);

        // query (and log) database version
        // let version = connection
        //     .query_one(Statement::from_string(Postgres, "SELECT version()"))
        //     .await
        //     .expect("cannot get pg version (exec sql fail)")
        //     .expect("cannot get pg version (no result)")
        //     .try_get_by_index::<String>(0)
        //     .expect("cannot get pg version (no column)");
        //
        // tracing::info!("Connected to the database ({version})");

        // apply migrations
        Self::migrate(database_url)
            .await
            .expect("cannot apply migrations");

        connection
    }
}
