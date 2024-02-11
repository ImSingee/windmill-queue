use crate::app::db::models::NewQueue;
use crate::app::db::schema::queue;
use crate::app::db::DB;
use crate::app::event_locker::EventLocker;
use crate::app::mqueue::Processor;
use crate::app::queue_events::{EventWithMeta, EventsMeta};
use crate::configuration::Config;
use anyhow::Result;
use apalis::layers::{Extension, TraceLayer};
use apalis::postgres::PostgresStorage;
use apalis::prelude::{Job, JobContext, Monitor, WithStorage, WorkerBuilder, WorkerFactoryFn};
use chrono::NaiveDateTime;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewEvents {
    pub events: Vec<EventWithMeta>,
    pub meta: EventsMeta,
}

impl Job for NewEvents {
    const NAME: &'static str = "wq::new_events";
}

pub type NewEventsProducer = PostgresStorage<NewEvents>;

async fn on_new_events(NewEvents { events, meta }: NewEvents, ctx: JobContext) -> Result<()> {
    let queue = meta.queue;
    let locker = ctx.data::<EventLocker>().unwrap().clone();

    tracing::debug!("acquire lock for queue {queue}");
    let _guard = locker.lock_for(queue.clone()).await;
    tracing::debug!("got lock for queue {queue}");

    tracing::info!("Received {} new events in queue {queue}", events.len());
    for event_with_meta in &events {
        tracing::debug!("Event: {:?}", event_with_meta);
    }

    if events.is_empty() {
        return Ok(());
    }

    let events: Vec<NewQueue> = events
        .into_iter()
        .map(|e| {
            let ts = NaiveDateTime::from_timestamp_millis(e.meta.ts as i64).unwrap();

            let event = json!(e.event);
            let meta = json!(e.meta);

            NewQueue {
                uuid: Uuid::now_v7(),
                ref_queue: queue.clone(),
                ref_id: e.meta.id,
                trace_id: e.meta.trace_id,
                event,
                meta,
                ts,
            }
        })
        .collect();

    let db = ctx.data::<DB>().unwrap();
    let mut conn = db.conn().await?;

    diesel::insert_into(queue::table)
        .values(events)
        .on_conflict((queue::ref_queue, queue::ref_id))
        .do_nothing()
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn new(config: Arc<Config>, db: DB) -> NewEventsProducer {
    let database_url = &config.database.url;
    let pg: PostgresStorage<NewEvents> = PostgresStorage::connect(database_url).await.unwrap();
    pg.setup()
        .await
        .expect("unable to run migrations for postgres");

    let storage = pg;
    let producer = storage.clone();

    let locker = EventLocker::new();
    let cloned_db = db.clone();
    tokio::spawn(async move {
        Monitor::new()
            .register_with_count(4, move |index| {
                // TODO recover from panic

                WorkerBuilder::new(format!("wq-new-events-worker-{index}"))
                    .layer(TraceLayer::new())
                    .layer(Extension(locker.clone()))
                    .layer(Extension(cloned_db.clone()))
                    .with_storage(storage.clone())
                    .build_fn(on_new_events)
            })
            .run()
            .await
            .expect("queue monitor run failed");
    });

    let processor = Processor::new(db.clone());
    processor.starts(4); // TODO support configure count

    producer
}
