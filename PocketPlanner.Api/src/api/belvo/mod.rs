mod token;
mod webhook;

use axum::{routing, Router};

pub fn router() -> axum::Router {
    Router::new()
        .route("/token", routing::post(token::handler))
        .route("/webhook", routing::post(webhook::handler))
}
