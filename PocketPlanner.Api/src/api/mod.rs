mod auth;
mod calculations;
mod user;

use axum::{http::StatusCode, response::IntoResponse, routing, Json, Router};
use serde_json::json;

pub fn router() -> axum::Router {
    Router::new()
        .route("/health", routing::get(|| async { "healthy!" }))
        .nest("/api", get_api_router())
}

fn get_api_router() -> Router {
    Router::new()
        .nest("/calculations", calculations::router())
        .nest(
            "/user",
            user::router().route_layer(axum::middleware::from_fn(auth::auth)),
        )
}

pub type ResponseResult<T> = Result<Json<T>, ResponseError>;

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("IO Error: {0}")]
    IO(#[from] mongodb::error::Error),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            ResponseError::IO(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (code, Json(json!({"error": self.to_string() }))).into_response()
    }
}
