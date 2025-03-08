use axum::Router;

pub mod transaction;

pub fn router() -> Router {
    Router::new().nest("/transaction", transaction::router())
}
