use axum::{
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
};

mod api;
pub mod infra;
mod util;

use serde_json::json;

pub use api::router;

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        AppError::Validation(rejection.body_text())
    }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct Json<T>(pub T);

impl<T: serde::Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

pub type Response<T> = Result<Json<T>, AppError>;
pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    Database(#[from] tokio_rusqlite::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
        };

        (code, Json(json!({"error": self.to_string() }))).into_response()
    }
}
