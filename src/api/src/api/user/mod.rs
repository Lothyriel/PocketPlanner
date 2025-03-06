use axum::{routing, Extension, Json, Router};

use auth::UserClaims;

mod auth;

pub fn router() -> Router {
    Router::new()
        .route("/summary", routing::get(handler))
        // TODO fix this refresh token endpoint and his location
        .route("/token", routing::post(auth::refresh))
        .route_layer(axum::middleware::from_fn(auth::auth))
}

pub async fn handler(Extension(user_claims): Extension<UserClaims>) -> Json<UserClaims> {
    Json(user_claims)
}
