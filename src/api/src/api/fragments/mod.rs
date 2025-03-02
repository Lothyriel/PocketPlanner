use axum::{
    response::{IntoResponse, Response},
    Router,
};

mod transaction;

pub fn router() -> Router {
    Router::new().nest("/transaction", transaction::router())
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        lib::templates::error(self.0).into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
