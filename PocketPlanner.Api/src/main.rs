use std::net::SocketAddr;

use log::error;
use pocket_planner::api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let server = axum::Server::bind(&addr).serve(api::router().into_make_service());

    if let Err(err) = server.await {
        error!("{}", err);
    }
}

#[allow(dead_code)]
enum ResponseError {}
