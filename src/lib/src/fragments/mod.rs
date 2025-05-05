use axum::Router;

use crate::infra::DbState;

pub mod transaction;

pub fn router(state: DbState) -> Router {
    Router::new().nest("/transaction", transaction::router(state))
}
