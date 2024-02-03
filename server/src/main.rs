mod app;
mod task;

use axum::{routing::{get, post}, Router, extract::{Json, Extension}};
use clap::{arg, command};
use serde_json::Value as JsonValue;
use clap::Parser;
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use crate::app::{App, TaskSender};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, env = "WQ_LISTEN", help = "The address to listen on", default_value = "0.0.0.0:8050")]
    listen: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let mut app = App::new();
    let task_sender = app.task_sender();
    let handle1 = tokio::spawn(async move {
        app.run().await
    });

    let router = Router::new()
        .route("/", get(root))
        .route("/demo", post(new_demo_task))
        .layer(Extension(task_sender))
        ;
    let handle2 = tokio::spawn(async move {
        println!("Listen on: {}", args.listen);
        let listener = TcpListener::bind(args.listen).await.unwrap();
        axum::serve(listener, router).await
    });

    tokio::try_join!(handle1, handle2).unwrap();
}

// basic handler that responds with a static string
async fn root() -> Json<JsonValue> {
    Json(json!({"success": true}))
}

#[derive(Deserialize)]
struct DemoTaskPayload {
    msg: String,
}

async fn new_demo_task(Json(payload): Json<DemoTaskPayload>, Extension(task_sender): Extension<TaskSender>) -> Json<JsonValue> {
    // task_sender.send(task::Task::Demo(msg)).await.unwrap();

    Json(json!({"success": true}))
}