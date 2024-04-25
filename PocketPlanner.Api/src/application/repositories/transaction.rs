use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};

use crate::application::model::transaction::Transaction;

use super::{DatabaseError, DbResult};

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

    pub async fn insert(&self, tx: &Transaction) -> DbResult<ObjectId> {
        let result = self.transactions.insert_one(tx, None).await?;

        let id = result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| DatabaseError::ExpectedObjectId)?;

        Ok(id)
    }

    pub async fn get_extract(&self, email: &str) -> DbResult<Vec<Transaction>> {
        let transactions = self
            .transactions
            .find(doc! { "email": email }, None)
            .await?
            .try_collect()
            .await?;

        Ok(transactions)
    }

    pub async fn delete(&self, email: &str, id: ObjectId) -> DbResult<()> {
        self.transactions
            .delete_one(doc! {"email" : email, "_id": id}, None)
            .await?;

        Ok(())
    }
}
