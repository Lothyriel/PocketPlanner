use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(get))
        .route("/", routing::post(add))
}

pub async fn handler(Extension(user_claims): Extension<UserClaims>) -> Json<String> {
    todo!()
}

pub async fn handler(Extension(user_claims): Extension<UserClaims>) -> Json<String> {
    todo!()
}
