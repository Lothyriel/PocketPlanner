use anyhow::Error;
use axum::Router;
use pocket_planner::extensions::log_ext::LogExt;

#[tokio::main]
async fn main() {
    start().await.log_err();
}

async fn start() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    Ok(axum::serve(listener, app).await?)
}
