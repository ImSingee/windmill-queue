use crate::app::db::models::Queue;
use crate::app::db::DB;
use thiserror::Error;

#[derive(Clone)]
pub struct Processor {
    db: DB,
}

impl Processor {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub fn starts(self, count: usize) {
        for _ in 0..count {
            let id = uuid::Uuid::now_v7().to_string();
            let processor = self.clone();
            tokio::spawn(async move {
                processor.start(id).await;
            });
        }
    }

    #[tracing::instrument("processor", level = "INFO", skip_all, fields(id = %id))]
    async fn start(&self, id: String) {
        loop {
            tracing::info!("Processor is running");

            // let event = self.get_event().await;

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    async fn get_event(&self) -> Option<Queue> {
        unimplemented!()

        // let result = self
        //     .db
        //     .transaction(|tx| -> ProcessorResult<Option<entity::Events>> async {
        //         // find the first event that is not locked
        //         // -> where: locked_at == null || locked_at < now - lock_timeout
        //
        //         entity::Events::update_many().filter().first().exec_with_returning()
        //
        //         let found = entity::Events::find()
        //             .filter(
        //                 Condition::any().add(Expr::col(events::Column::UpdatedAt.is_null())), // .add(Expr::col((Events::Table, Glyph::Image)).like("A%")))
        //             )
        //             .one(tx)
        //
        //             .await?;
        //
        //         if found.is_none() {
        //             return Ok(None);
        //         }
        //
        //         Ok(None) // TODO
        //     })
        //     .await;
        //
        // result.unwrap_or_else(|_| None)
    }

    async fn mark_event_as_processed(&self, event: Queue) -> Result<(), ProcessorError> {
        unimplemented!()
    }
}

pub type ProcessorResult<T> = Result<T, ProcessorError>;
#[derive(Debug, Error)]
pub enum ProcessorError {}
