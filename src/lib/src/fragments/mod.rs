use anyhow::Error;
use axum::{response::IntoResponse, Router};

pub mod transaction;

pub fn router() -> Router {
    Router::new().nest("/transaction", transaction::router())
}

pub fn error(error: Error) -> ErrorTemplate {
    ErrorTemplate { error }
}

#[derive(askama::Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    error: Error,
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> askama_axum::Response {
        error(self.0).into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
