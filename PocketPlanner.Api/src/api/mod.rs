mod calculations;

use axum::{routing, Router};

pub fn router() -> axum::Router {
    Router::new()
        .route("/health", routing::get(|| async { "healthy!" }))
        .nest("/api", get_api_router())
}

fn get_api_router() -> Router {
    Router::new().nest("/calculations", calculations::router())
}
