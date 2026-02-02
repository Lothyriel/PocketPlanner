use axum::Router;

use crate::infra::DbState;

pub mod card;
pub mod category;
pub mod transaction;

pub fn router(state: DbState) -> Router {
    Router::new()
        .nest("/card", card::router(state.clone()))
        .nest("/category", category::router(state.clone()))
        .nest("/transaction", transaction::router(state))
}
