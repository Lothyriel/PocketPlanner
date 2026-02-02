mod calculations;
mod user;

use axum::{Router, routing};

use crate::application::ApiState;

pub use user::{auth, get_google_jwks};

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/health", routing::get(|| async { "healthy!" }))
        .nest("/calculations", calculations::router())
        .nest("/user", user::router(state))
}
