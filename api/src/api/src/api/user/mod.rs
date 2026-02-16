use axum::{Extension, Json, Router, routing};
use lib::infra::UserClaims;

use crate::application::ApiState;

mod auth;

pub use auth::{auth, get_google_jwks};

pub fn router(state: ApiState) -> Router {
    let auth = axum::middleware::from_fn_with_state(state.clone(), auth::auth);

    Router::new()
        .route("/me", routing::get(handler))
        .route_layer(auth)
        .route("/session/refresh", routing::post(auth::refresh))
        .route("/session", routing::post(auth::login))
        .route("/session", routing::delete(auth::logout))
        .with_state(state)
}

pub async fn handler(Extension(user_claims): Extension<UserClaims>) -> Json<UserClaims> {
    Json(user_claims)
}
