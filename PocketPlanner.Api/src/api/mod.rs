mod auth;
mod calculations;
mod user;

use axum::{http::StatusCode, response::IntoResponse, routing, Json, Router};
use mongodb::error::Error;
use serde_json::json;

use crate::application::repositories::{get_mongo_client, transaction::TransactionRepository};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", routing::get(|| async { "healthy!" }))
        .nest("/api", get_api_router(state))
}

fn get_api_router(state: AppState) -> Router {
    Router::new()
        .nest("/calculations", calculations::router())
        .nest(
            "/user",
            user::router(state).route_layer(axum::middleware::from_fn(auth::auth)),
        )
}

type ResponseResult<T> = Result<Json<T>, ResponseError>;

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

#[derive(Clone)]
pub struct AppState {
    pub transactions: TransactionRepository,
}

pub async fn get_state() -> Result<AppState, Error> {
    let database = get_mongo_client().await?.database("pocket-planner");

    let state = AppState {
        transactions: TransactionRepository::new(&database),
    };

    Ok(state)
}
