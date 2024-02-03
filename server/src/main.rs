use axum::{
    routing::get,
    Router,
};
use clap::{arg, command};

use clap::Parser;
use tokio::net::TcpListener;

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

    let app = Router::new()
        .route("/", get(root));


    println!("Listen on: {}", args.listen);

    let listener = TcpListener::bind(args.listen).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}