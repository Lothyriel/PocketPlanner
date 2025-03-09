use axum::{routing, Extension, Json, Router};

use auth::UserClaims;

use crate::application::AppState;

mod auth;

pub use auth::get_google_jwks;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/summary", routing::get(handler))
        // TODO fix this refresh token endpoint and his location
        .route("/token", routing::post(auth::refresh))
        .route_layer(axum::middleware::from_fn_with_state(state, auth::auth))
}

pub async fn handler(Extension(user_claims): Extension<UserClaims>) -> Json<UserClaims> {
    Json(user_claims)
}
