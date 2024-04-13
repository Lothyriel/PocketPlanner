use futures::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

use crate::{api::ResponseError, application::model::transaction::Transaction};

pub struct TransactionRepository {
    transactions: Collection<Transaction>,
}

impl TransactionRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            transactions: database.collection("Transactions"),
        }
    }

    pub async fn insert(&self, tx: Transaction) -> Result<(), ResponseError> {
        self.transactions.insert_one(tx, None).await?;

        Ok(())
    }

    pub async fn get_extract(&self, email: String) -> Result<Vec<Transaction>, ResponseError> {
        let cursor = self
            .transactions
            .find(doc! { "email": email }, None)
            .await?;

        let transactions = cursor.try_collect().await?;

        Ok(transactions)
    }
}
