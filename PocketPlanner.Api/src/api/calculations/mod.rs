mod wage_deduction;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new().route(
        "/wage-deduction/:wage",
        routing::get(wage_deduction::handler),
    )
}
