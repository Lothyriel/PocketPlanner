use anyhow::{Error, Result};
use askama_web::WebTemplate;
use axum::{
    response::{IntoResponse, Response},
    Router,
};
use rusqlite::Connection;

mod fragments;
mod views;

pub fn router() -> Router {
    views::router().nest("/fragments", fragments::router())
}

fn error(error: Error) -> ErrorTemplate {
    ErrorTemplate { error }
}

#[derive(askama::Template, WebTemplate)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    error: Error,
}

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error(self.0).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

pub fn connect_db() -> Result<Connection> {
    Ok(Connection::open("user.db")?)
}

pub fn init_db() -> Result<()> {
    let conn = connect_db()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id          INTEGER PRIMARY KEY,
            amount      INTEGER NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}
