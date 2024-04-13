use futures::TryStreamExt;
use mongodb::{bson::doc, error::Result, Collection, Database};

use crate::application::model::transaction::Transaction;

#[derive(Clone)]
pub struct TransactionRepository {
    transactions: Collection<Transaction>,
}

impl TransactionRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            transactions: database.collection("Transactions"),
        }
    }

    pub async fn insert(&self, tx: Transaction) -> Result<()> {
        self.transactions.insert_one(tx, None).await?;

        Ok(())
    }

    pub async fn get_extract(&self, email: String) -> Result<Vec<Transaction>> {
        self.transactions
            .find(doc! { "email": email }, None)
            .await?
            .try_collect()
            .await
    }
}
