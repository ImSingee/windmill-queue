use crate::app::db::Connection;

pub struct Processor {
    db: Connection,
}

impl Processor {
    pub fn new(db: Connection) -> Self {
        Self { db }
    }

    pub async fn start(&self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            tracing::info!("Processor is running");
        }
    }
}
