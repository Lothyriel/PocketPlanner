use std::sync::Arc;
use tokio_rusqlite::Connection;

pub mod card;
pub mod category;
pub mod db;
pub mod transaction;

pub use card::{Card, CardType, CreateCard, UpdateCard};
pub use category::{Category, CreateCategory};
pub use db::init_db;
pub use transaction::{CreateTransaction, Transaction, TransactionType};

#[derive(Clone)]
pub struct DbState {
    pub conn: Arc<Connection>,
}

impl DbState {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(conn),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct UserClaims {
    pub email: String,
    pub name: String,
    pub picture: String,
}
