pub mod assets;
mod auth;
mod calculations;
mod user;

use axum::{http::StatusCode, response::IntoResponse, routing, Json, Router};
use serde_json::json;

use crate::{
    application::repositories::{
        get_mongo_client, transaction::TransactionRepository, DatabaseError,
    },
    AppState,
};

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
