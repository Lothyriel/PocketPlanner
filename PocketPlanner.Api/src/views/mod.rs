use axum::{routing, Router};

use crate::AppState;

pub fn router(_state: AppState) -> Router {
    Router::new().route("/", routing::get(index))
}

async fn index() -> IndexTemplate {
    IndexTemplate { test: TestTemplate }
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    test: TestTemplate,
}

#[derive(askama::Template)]
#[template(path = "test.html")]
struct TestTemplate;
