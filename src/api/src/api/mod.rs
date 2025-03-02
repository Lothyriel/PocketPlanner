mod auth;
mod calculations;
mod fragments;
mod user;

use axum::{routing, Router};
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new()
        .route("/health", routing::get(|| async { "healthy!" }))
        .nest("/api", api_router())
        .nest("/fragments", fragments::router())
        .fallback_service(ServeDir::new("public"))
}

fn api_router() -> Router {
    Router::new()
        // TODO fix this refresh token endpoint and his location
        .route("/token", routing::post(auth::refresh))
        .nest("/calculations", calculations::router())
        .nest(
            "/user",
            user::router().route_layer(axum::middleware::from_fn(auth::auth)),
        )
}
