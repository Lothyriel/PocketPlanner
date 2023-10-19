use axum::{routing, Router};
use log::error;

#[tokio::main]
async fn main() {
    if let Err(err) = start().await {
        error!("{}", err);
    }
}

async fn start() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/health", routing::get(|| async { "healthy!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await
}

enum ResponseError {}
