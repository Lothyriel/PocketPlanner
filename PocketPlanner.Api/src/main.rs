#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));

    let state = pocket_planner::get_state()
        .await
        .expect("Failed to initialize application state");

    let router = pocket_planner::router(state).into_make_service();

    let server = axum::Server::bind(&addr).serve(router);

    log::info!("Starting API in: {}", addr);

    if let Err(err) = server.await {
        log::error!("{}", err);
    }
}
