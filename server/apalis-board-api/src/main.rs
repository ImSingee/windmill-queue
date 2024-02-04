use std::collections::HashSet;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Scope};
use apalis::{
    postgres::PostgresStorage,
    prelude::*,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wq_server::app::queue::NewEvents;

#[derive(Serialize)]
struct JobsResult<J> {
    jobs: Vec<JobRequest<J>>,
    counts: JobStateCount,
}

#[derive(Deserialize)]
struct Filter {
    #[serde(default)]
    status: JobState,
    #[serde(default)]
    page: i32,
}

async fn push_job<J, S>(job: web::Json<J>, storage: web::Data<S>) -> HttpResponse
    where
        J: Job + Serialize + DeserializeOwned + 'static,
        S: Storage<Output=J>,
{
    let storage = &*storage.into_inner();
    let mut storage = storage.clone();
    let res = storage.push(job.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().body(format!("Job with ID [{id}] added to queue")),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}

async fn get_jobs<J, S>(storage: web::Data<S>, filter: web::Query<Filter>) -> HttpResponse
    where
        J: Job + Serialize + DeserializeOwned + 'static,
        S: Storage<Output=J> + JobStreamExt<J> + Send,
{
    let storage = &*storage.into_inner();
    let mut storage = storage.clone();
    let counts = storage.counts().await.unwrap();
    let jobs = storage.list_jobs(&filter.status, filter.page).await;

    match jobs {
        Ok(jobs) => HttpResponse::Ok().json(JobsResult { jobs, counts }),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}

async fn get_workers<J, S>(storage: web::Data<S>) -> HttpResponse
    where
        J: Job + Serialize + DeserializeOwned + 'static,
        S: Storage<Output=J> + JobStreamExt<J>,
{
    let storage = &*storage.into_inner();
    let mut storage = storage.clone();
    let workers = storage.list_workers().await;
    match workers {
        Ok(workers) => HttpResponse::Ok().json(serde_json::to_value(workers).unwrap()),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}

async fn get_job<J, S>(job_id: web::Path<JobId>, storage: web::Data<S>) -> HttpResponse
    where
        J: Job + Serialize + DeserializeOwned + 'static,
        S: Storage<Output=J> + 'static,
{
    let storage = &*storage.into_inner();
    let storage = storage.clone();
    let res = storage.fetch_by_id(&job_id).await;
    match res {
        Ok(Some(job)) => HttpResponse::Ok().json(job),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}

trait StorageRest<J>: Storage<Output=J> {
    fn name(&self) -> String;
}

impl<J, S> StorageRest<J> for S
    where
        S: Storage<Output=J> + JobStreamExt<J> + 'static,
        J: Job + Serialize + DeserializeOwned + 'static,
{
    fn name(&self) -> String {
        J::NAME.to_string()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Queue {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct QueueList {
    set: HashSet<String>,
}

struct StorageApiBuilder {
    scope: Scope,
    list: QueueList,
}

impl StorageApiBuilder {
    fn add_storage<J, S>(mut self, storage: S) -> Self
        where
            J: Job + Serialize + DeserializeOwned + 'static,
            S: StorageRest<J> + JobStreamExt<J>,
            S: Storage<Output=J>,
            S: 'static + Send,
    {
        let name = J::NAME.to_string();
        self.list.set.insert(name);

        Self {
            scope: self.scope.service(
                Scope::new(J::NAME)
                    .app_data(web::Data::new(storage))
                    .route("", web::get().to(get_jobs::<J, S>)) // Fetch jobs in queue
                    .route("/workers", web::get().to(get_workers::<J, S>)) // Fetch jobs in queue
                    .route("/job", web::put().to(push_job::<J, S>)) // Allow add jobs via api
                    .route("/job/{job_id}", web::get().to(get_job::<J, S>)), // Allow fetch specific job
            ),
            list: self.list,
        }
    }

    fn build(self) -> Scope {
        async fn fetch_queues(queues: web::Data<QueueList>) -> HttpResponse {
            let mut queue_result = Vec::new();
            for queue in &queues.set {
                queue_result.push(Queue {
                    name: queue.clone(),
                })
            }
            #[derive(Serialize)]
            struct Res {
                queues: Vec<Queue>,
            }

            HttpResponse::Ok().json(Res {
                queues: queue_result,
            })
        }

        self.scope
            .app_data(web::Data::new(self.list))
            .route("", web::get().to(fetch_queues))
    }

    fn new() -> Self {
        Self {
            scope: Scope::new("queues"),
            list: QueueList {
                set: HashSet::new(),
            },
        }
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug,sqlx::query=error");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("Must specify DATABASE_URL");
    let pg: PostgresStorage<NewEvents> = PostgresStorage::connect(database_url).await?;


    HttpServer::new(move || {
        App::new().wrap(Cors::permissive()).service(
            web::scope("/api").service(
                StorageApiBuilder::new()
                    .add_storage(pg.clone())
                    .build(),
            ),
        )
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await?;

    Ok(())
}
