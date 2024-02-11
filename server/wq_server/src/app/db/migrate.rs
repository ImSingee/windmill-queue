use diesel::prelude::*;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use super::error::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/app/db/migrations");

pub async fn run_migrations(database_url: &str) -> Result<()> {
    let database_url = database_url.to_string();

    tokio::task::spawn_blocking(move || {
        let mut conn = AsyncConnectionWrapper::<AsyncPgConnection>::establish(&database_url)?;

        conn.run_pending_migrations(MIGRATIONS)?;

        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    })
    .await??;

    Ok(())
}
