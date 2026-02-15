pub mod installments;
pub mod savings;
mod wage_deduction;

use axum::{Router, routing};

pub fn router() -> Router {
    Router::new()
        .route("/wage-deduction", routing::get(wage_deduction::handler))
        .route("/installments", routing::get(installments::handler))
        .route("/savings", routing::get(savings::handler))
}

pub const SELIC: f64 = 15.0;
pub const CDI: f64 = SELIC - 0.1;
