use anyhow::Result;
use rusqlite::Connection;

pub mod fragments;

pub fn connect_db() -> Result<Connection> {
    Ok(Connection::open("user.db")?)
}

pub fn init_db() -> Result<()> {
    let conn = connect_db()?;

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
