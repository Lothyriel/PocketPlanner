use std::net::SocketAddr;

use log::{error, info};
use pocket_planner::api;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let server = axum::Server::bind(&addr).serve(api::router().into_make_service());

    info!("Starting API in: {}", addr);

    if let Err(err) = server.await {
        error!("{}", err);
    }
}

#[allow(dead_code)]
enum ResponseError {}
