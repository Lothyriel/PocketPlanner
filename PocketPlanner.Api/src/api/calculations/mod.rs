mod installments;
mod wage_deduction;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route("/wage-deduction", routing::get(wage_deduction::handler))
        .route("/installments", routing::get(installments::handler))
}
