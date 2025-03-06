mod calculations;
mod user;

use axum::{routing, Router};
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new()
        .route("/health", routing::get(|| async { "healthy!" }))
        .nest("/api", api_router())
        .nest("/fragments", lib::fragments::router())
        .fallback_service(ServeDir::new("public"))
}

fn api_router() -> Router {
    Router::new()
        .nest("/calculations", calculations::router())
        .nest("/user", user::router())
}
