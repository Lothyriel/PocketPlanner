use std::sync::Arc;

use application::AppState;
use axum::{response::IntoResponse, Json, Router};
use reqwest::StatusCode;
use serde_json::json;
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
                .unwrap_or("debug,hyper=off".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));

    let jwkset = api::get_google_jwks().await.expect("Get google JWKset");

    let state = AppState {
        google_keys: Arc::new(RwLock::new(jwkset)),
    };

    let router = router(state).layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("To bind to {addr:?}");

    let server = axum::serve(listener, router);

    tracing::info!("Starting API in: {}", addr);

    if let Err(err) = server.await {
        tracing::error!("{}", err);
    }
}

pub fn router(state: AppState) -> Router {
    lib::router()
        .nest("/api", api::router(state))
        .fallback_service(ServeDir::new("public"))
}

type ResponseResult<T> = Result<Json<T>, ResponseError>;

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("HttpError: {0}")]
    Http(#[from] reqwest::Error),
    #[error("EnvError: {0}")]
    Environment(#[from] std::env::VarError),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            ResponseError::Http(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::Environment(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (code, Json(json!({"error": self.to_string() }))).into_response()
    }
}
