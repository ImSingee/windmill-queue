use crate::app::event_locker::EventLocker;
use crate::app::queue_events::{EventWithMeta, EventsMeta};
use crate::configuration::Config;
use apalis::layers::Extension;
use apalis::postgres::PostgresStorage;
use apalis::prelude::{Job, JobContext, Monitor, WithStorage, WorkerBuilder, WorkerFactoryFn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewEvents {
    pub events: Vec<EventWithMeta>,
    pub meta: EventsMeta,
}

impl Job for NewEvents {
    const NAME: &'static str = "wq::new_events";
}

pub type NewEventsProducer = PostgresStorage<NewEvents>;

async fn on_new_events(NewEvents { events, meta }: NewEvents, ctx: JobContext) {
    let queue = meta.queue;
    let locker = ctx.data::<EventLocker>().unwrap().clone();

    tracing::debug!("acquire lock for queue {queue}");
    let guard = locker.lock_for(queue.clone()).await;
    tracing::debug!("got lock for queue {queue}");

    // TODO

    tracing::info!("Received {} new events in queue {queue}", events.len());

    for event_with_meta in events {
        tracing::info!("Event: {:?}", event_with_meta);
    }

    // tracing::info!("start handling...");
    // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    // tracing::info!("end handling...");

    drop(guard);
}

pub async fn new(config: Config) -> NewEventsProducer {
    let database_url = config.database.url;
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
                WorkerBuilder::new(format!("wq-new-events-worker-{index}"))
                    .layer(Extension(locker.clone()))
                    .with_storage(storage.clone())
                    .build_fn(on_new_events)
            })
            .run()
            .await
            .expect("queue monitor run failed");
    });

    producer
}
