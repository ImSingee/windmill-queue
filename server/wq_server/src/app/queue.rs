use crate::app::db::Connection;
use crate::app::event_locker::EventLocker;
use crate::app::queue_events::{EventWithMeta, EventsMeta};
use crate::configuration::Config;
use anyhow::Result;
use apalis::layers::Extension;
use apalis::postgres::PostgresStorage;
use apalis::prelude::{Job, JobContext, Monitor, WithStorage, WorkerBuilder, WorkerFactoryFn};
use chrono::NaiveDateTime;
use entity::events;
use sea_orm::{ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

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
    let guard = locker.lock_for(queue.clone()).await;
    tracing::debug!("got lock for queue {queue}");

    tracing::info!("Received {} new events in queue {queue}", events.len());
    for event_with_meta in &events {
        tracing::debug!("Event: {:?}", event_with_meta);
    }

    if events.is_empty() {
        return Ok(());
    }

    let events = events.into_iter().map(|e| {
        let ts = NaiveDateTime::from_timestamp_millis(e.meta.ts as i64).unwrap();

        let event = json!(e.event);
        let meta = json!(e.meta);

        events::ActiveModel {
            queue: ActiveValue::Set(queue.clone()),
            id: ActiveValue::Set(e.meta.id),
            trace_id: ActiveValue::Set(e.meta.trace_id),
            event: ActiveValue::Set(event),
            meta: ActiveValue::Set(meta),
            ts: ActiveValue::Set(ts),
            ..Default::default()
        }
    });

    let db = ctx.data::<Connection>().unwrap();

    // TODO filter out duplicate events

    events::Entity::insert_many(events)
        .on_empty_do_nothing()
        .exec(db.inner())
        .await?;

    drop(guard);
    Ok(())
}

pub async fn new(config: Arc<Config>, db: Connection) -> NewEventsProducer {
    let database_url = &config.database.url;
    let pg: PostgresStorage<NewEvents> = PostgresStorage::connect(database_url).await.unwrap();
    pg.setup()
        .await
        .expect("unable to run migrations for postgres");

    let storage = pg;
    let producer = storage.clone();

    let locker = EventLocker::new();

    tokio::spawn(async move {
        Monitor::new()
            .register_with_count(4, move |index| {
                // TODO recover from panic

                WorkerBuilder::new(format!("wq-new-events-worker-{index}"))
                    .layer(Extension(locker.clone()))
                    .layer(Extension(db.clone()))
                    .with_storage(storage.clone())
                    .build_fn(on_new_events)
            })
            .run()
            .await
            .expect("queue monitor run failed");
    });

    producer
}
