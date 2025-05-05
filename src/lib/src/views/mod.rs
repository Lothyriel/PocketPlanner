use askama::Template;
use askama_web::WebTemplate;
use axum::{extract::State, response::IntoResponse, routing, Extension, Router};

use crate::{
    fragments::*,
    infra::{DbState, UserClaims},
    AppError,
};

pub fn router(state: DbState) -> Router {
    Router::new()
        .route("/", routing::get(home))
        .route("/transactions", routing::get(view_transactions))
        .with_state(state)
}

async fn home() -> IndexTemplate<HomeTemplate> {
    IndexTemplate {
        content: HomeTemplate,
    }
}

async fn view_transactions(
    state: State<DbState>,
    claims: Extension<UserClaims>,
) -> Result<impl IntoResponse, AppError> {
    //todo: maybe add this to a macro to check if the route wants HTMX fragment or whole page reload
    let index = IndexTemplate {
        content: transaction::view(state, claims).await?,
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
