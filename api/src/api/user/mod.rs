mod transaction;

use axum::{routing, Extension, Json, Router};

use super::{auth::UserClaims, AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/summary", routing::get(handler))
        .nest("/transaction", transaction::router(state))
}

pub async fn handler(Extension(user_claims): Extension<UserClaims>) -> Json<UserClaims> {
    Json(user_claims)
}
