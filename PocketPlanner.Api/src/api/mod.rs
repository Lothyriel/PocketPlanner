pub mod assets;
mod auth;
mod calculations;
mod user;

use axum::{routing, Router};

use crate::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", routing::get(|| async { "healthy!" }))
        .route("/token", routing::post(auth::refresh))
        .nest("/calculations", calculations::router())
        .nest(
            "/user",
            user::router(state).route_layer(axum::middleware::from_fn(auth::auth)),
        )
}
