use axum::{routing, Extension, Json, Router};
use lib::infra::UserClaims;

use crate::application::ApiState;

mod auth;

pub use auth::{auth, get_google_jwks};

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/summary", routing::get(handler))
        // TODO fix this refresh token endpoint and his location
        .route("/token", routing::post(auth::refresh))
        .route_layer(axum::middleware::from_fn_with_state(state, auth::auth))
}

pub async fn handler(Extension(user_claims): Extension<UserClaims>) -> Json<UserClaims> {
    Json(user_claims)
}
