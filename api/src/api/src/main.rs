use std::{
    net::{Ipv6Addr, SocketAddr},
    sync::Arc,
};

use application::ApiState;
use axum::Router;
use lib::infra::{DbState, init_db};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod application;
mod extensions;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("debug,hyper=warn,rustls=warn,tungstenite=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let addr = SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 8080);

    let jwkset = api::get_google_jwks().await.expect("Get google JWKset");

    let audiences = std::env::var("G_CLIENT_IDS")
        .ok()
        .map(|value| {
            value
                .split(',')
                .map(|entry| entry.trim())
                .filter(|entry| !entry.is_empty())
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
        })
        .filter(|items| !items.is_empty())
        .expect("oauth audiences not configured");

    tracing::info!("Loaded G_CLIENT_IDS: {:?}", audiences);

    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "pocketplanner.db".to_string());
    let conn = init_db(&db_path).await.expect("Initialize database");
    let state = DbState::new(conn);

    let api_state = ApiState {
        google_keys: Arc::new(RwLock::new(jwkset)),
        audiences,
    };

    let router = router(state, api_state).layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("To bind to {addr:?}");

    let server = axum::serve(listener, router);

    tracing::info!("Starting API in: {}", addr);

    if let Err(err) = server.await {
        tracing::error!("{}", err);
    }
}

#[allow(unused_macros)]
macro_rules! expect_env {
    ($var_name:expr) => {
        std::env::var($var_name).expect(concat!("env missing: ", $var_name))
    };
}

pub fn router(state: DbState, api_state: ApiState) -> Router {
    let auth_layer = axum::middleware::from_fn_with_state(api_state.clone(), api::auth);

    lib::router(state)
        .layer(auth_layer)
        .merge(api::router(api_state))
        .fallback_service(ServeDir::new("public"))
}
