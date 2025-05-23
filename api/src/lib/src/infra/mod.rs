pub mod transaction;

use surrealdb::{engine::any::Any, Surreal};

use crate::AppResult;

pub type Db = Surreal<Any>;

#[derive(Clone)]
pub struct DbState {
    db: Db,
}

impl DbState {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn db(&self, user_id: &str) -> AppResult<&Db> {
        self.db.use_db(user_id).await?;
        Ok(&self.db)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct UserClaims {
    pub email: String,
    pub name: String,
    pub picture: String,
}
