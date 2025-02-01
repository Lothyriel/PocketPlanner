mod auth;
mod calculations;
mod user;

use axum::{routing, Router};
use tower_http::services::ServeDir;

use crate::application::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", routing::get(|| async { "healthy!" }))
        // TODO fix this refresh token endpoint and his location
        .route("/token", routing::post(auth::refresh))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest("/calculations", calculations::router())
        .nest(
            "/user",
            user::router(state).route_layer(axum::middleware::from_fn(auth::auth)),
        )
}
