use anyhow::{Error, Result};
use rusqlite::Connection;

pub mod transaction;

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            amount      INTEGER NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn error(error: Error) -> ErrorTemplate {
    ErrorTemplate { error }
}

#[derive(askama::Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    error: Error,
}
