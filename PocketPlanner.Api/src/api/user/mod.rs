mod transaction;

use axum::{routing, Extension, Json, Router};

use super::auth::UserClaims;

pub fn router() -> Router {
    Router::new()
        .route("/summary", routing::get(handler))
        .nest("/transaction", transaction::router())
}

pub async fn handler(Extension(user_claims): Extension<UserClaims>) -> Json<String> {
    Json(user_claims.email)
}
