mod auth;
pub mod calculations;
mod user;

use axum::{routing, Router};

pub fn router() -> axum::Router {
    Router::new()
        .route("/health", routing::get(|| async { "healthy!" }))
        .nest("/api", get_api_router())
}

fn get_api_router() -> Router {
    Router::new()
        .nest("/calculations", calculations::router())
        .nest(
            "/user",
            user::router().route_layer(axum::middleware::from_fn(auth::auth)),
        )
}
