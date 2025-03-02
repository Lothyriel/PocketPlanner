use axum::{response::IntoResponse, routing, Form, Router};

use lib::templates::{self, transaction::Transaction};

use crate::application::connect_db;

use super::AppError;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(view))
        .route("/add", routing::post(action))
}

async fn view() -> Result<impl IntoResponse, AppError> {
    let mut conn = connect_db()?;

    Ok(templates::transaction::view(&mut conn)?)
}

async fn action(Form(tx): Form<Transaction>) -> Result<impl IntoResponse, AppError> {
    let mut conn = connect_db()?;

    Ok(templates::transaction::action(&mut conn, tx)?)
}
