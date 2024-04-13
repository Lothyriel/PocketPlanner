use pocket_planner::api;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    let state = api::get_state()
        .await
        .expect("Failed to initialize application state");

    let router = api::router(state).into_make_service();

    let server = axum::Server::bind(&addr).serve(router);

    log::info!("Starting API in: {}", addr);

    if let Err(err) = server.await {
        log::error!("{}", err);
    }
}
