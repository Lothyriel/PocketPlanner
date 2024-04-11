use axum::{routing, Extension, Json, Router};

use crate::api::auth::UserClaims;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(get))
        .route("/", routing::post(add))
}

pub async fn get(Extension(user_claims): Extension<UserClaims>) -> Json<String> {
    todo!()
}

pub async fn add(Extension(user_claims): Extension<UserClaims>) -> Json<String> {
    todo!()
}
