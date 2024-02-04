use apalis::postgres::PostgresStorage;
use apalis::prelude::{Job, JobContext, Monitor, WithStorage, WorkerBuilder, WorkerFactoryFn};
use serde::{Deserialize, Serialize};
use crate::app::event::EventWithMeta;
use crate::configuration::Config;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewEvents {
    pub(crate) events_with_meta: Vec<EventWithMeta>,
}

impl Job for NewEvents {
    const NAME: &'static str = "wq::new_events";
}

pub type NewEventsProducer = PostgresStorage<NewEvents>;

async fn on_new_events(NewEvents { events_with_meta }: NewEvents, _ctx: JobContext) {
    // TODO

    println!("Received {} new events", events_with_meta.len());

    for event_with_meta in events_with_meta {
        println!("Event: {:?}", event_with_meta);
    }
}

pub async fn new(config: Config) -> NewEventsProducer {
    let database_url = config.database.url;
    let pg: PostgresStorage<NewEvents> = PostgresStorage::connect(database_url).await.unwrap();
    pg.setup()
        .await
        .expect("unable to run migrations for postgres");

    let storage = pg;
    let producer = storage.clone();

    tokio::spawn(async move {
        Monitor::new()
            .register_with_count(4, move |index| {
                WorkerBuilder::new(format!("wq-new-events-worker-{index}"))
                    .with_storage(storage.clone())
                    .build_fn(on_new_events)
            })
            .run()
            .await
            .expect("queue monitor run failed");
    });


    producer
}