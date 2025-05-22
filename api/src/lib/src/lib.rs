use anyhow::Error;
use askama_web::WebTemplate;
use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    response::{IntoResponse, Response},
    Router,
};
use infra::DbState;

mod fragments;
pub mod infra;
mod views;

pub fn router(state: DbState) -> Router {
    views::router(state.clone()).nest("/fragments", fragments::router(state))
}

fn error(error: Error) -> ErrorTemplate {
    ErrorTemplate { error }
}

#[derive(askama::Template, WebTemplate)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    error: Error,
}

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error(self.0).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        error(rejection)
    }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct Json<T>(T);

impl<T: serde::Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}
