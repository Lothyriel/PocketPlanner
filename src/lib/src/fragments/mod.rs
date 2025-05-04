use axum::Router;

use crate::AppState;

pub mod transaction;

pub fn router(state: AppState) -> Router {
    Router::new().nest("/transaction", transaction::router(state))
}
