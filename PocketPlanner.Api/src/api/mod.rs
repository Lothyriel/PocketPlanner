mod assets;
mod auth;
mod belvo;
mod calculations;
mod user;

use axum::{http::StatusCode, response::IntoResponse, routing, Json, Router};
use mongodb::error::Error;
use serde_json::json;

use crate::application::repositories::{
    get_mongo_client, transaction::TransactionRepository, DatabaseError,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/.well-known", assets::asset_links_router())
        .nest("/api", get_api_router(state))
}

fn get_api_router(state: AppState) -> Router {
    Router::new()
        .route("/health", routing::get(|| async { "healthy!" }))
        .route("/token", routing::post(auth::refresh))
        .nest("/calculations", calculations::router())
        .nest("/belvo", belvo::router())
        .nest(
            "/user",
            user::router(state).route_layer(axum::middleware::from_fn(auth::auth)),
        )
}

type ResponseResult<T> = Result<Json<T>, ResponseError>;

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("DatabaseError: {0}")]
    Database(#[from] DatabaseError),
    #[error("HttpError: {0}")]
    Http(#[from] reqwest::Error),
    #[error("EnvError: {0}")]
    Environment(#[from] std::env::VarError),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            ResponseError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::Http(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::Environment(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
