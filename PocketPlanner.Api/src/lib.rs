use application::repositories::{
    get_mongo_client, transaction::TransactionRepository, DatabaseError,
};
use axum::{response::IntoResponse, Json, Router};
use mongodb::error::Error;
use reqwest::StatusCode;
use serde_json::json;

mod api;
mod application;
mod extensions;
mod views;

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

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/.well-known", api::assets::asset_links_router())
        .nest("/api", api::router(state.clone()))
        .nest("/", views::router(state))
}

pub async fn init_state() -> Result<AppState, Error> {
    let database = get_mongo_client().await?.database("pocket-planner");

    let state = AppState {
        transactions: TransactionRepository::new(&database),
    };

    Ok(state)
}
