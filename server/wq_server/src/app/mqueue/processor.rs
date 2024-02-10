use crate::app::db::Connection;

#[derive(Clone)]
pub struct Processor {
    db: Connection,
}

impl Processor {
    pub fn new(db: Connection) -> Self {
        Self { db }
    }

    pub fn starts(self, count: usize) {
        for i in 1..=count {
            let processor = self.clone();
            tokio::spawn(async move {
                processor.start(i).await;
            });
        }
    }

    #[tracing::instrument("processor", level = "INFO", skip_all, fields(id = %id))]
    async fn start(&self, id: usize) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            tracing::info!("Processor {id} is running");
        }
    }
}
