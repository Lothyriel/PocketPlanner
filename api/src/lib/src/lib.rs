use std::sync::Arc;

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
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("rusqlite error: {0}")]
    Database(#[from] tokio_rusqlite::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg, err) = match self {
            Self::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
                Some(self),
            ),
            Self::Validation(reason) => (StatusCode::BAD_REQUEST, reason, None),
        };

        let mut response = (code, Json(json!({"error": msg }))).into_response();

        if let Some(err) = err {
            // Insert our error into the response, our logging middleware will use this.
            // By wrapping the error in an Arc we can use it as an Extension regardless of any inner types not deriving Clone.
            response.extensions_mut().insert(Arc::new(err));
        }

        response
    }
}
