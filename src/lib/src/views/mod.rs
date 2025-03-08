use askama::Template;
use askama_web::WebTemplate;
use axum::{response::IntoResponse, routing, Router};

use crate::{fragments::*, AppError};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(home))
        .route("/transactions", routing::get(view_transactions))
}

async fn home() -> IndexTemplate<HomeTemplate> {
    IndexTemplate {
        content: HomeTemplate,
    }
}

async fn view_transactions() -> Result<impl IntoResponse, AppError> {
    let index = IndexTemplate {
        content: transaction::view().await?,
    };

    Ok(index)
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct IndexTemplate<T: Template> {
    content: T,
}

#[derive(Template, WebTemplate)]
#[template(path = "home.html")]
struct HomeTemplate;
