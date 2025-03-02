use rusqlite::Connection;

pub mod extractors;
pub mod model;

pub fn connect_db() -> anyhow::Result<Connection> {
    Ok(Connection::open("user.db")?)
}
