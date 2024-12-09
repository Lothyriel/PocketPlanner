#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));

    let state = pocket_planner::init_state()
        .await
        .expect("To initialize application state");

    let router = pocket_planner::router(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("To bind to {addr:?}");

    let server = axum::serve(listener, router);

    log::info!("Starting API in: {}", addr);

    if let Err(err) = server.await {
        log::error!("{}", err);
    }
}
