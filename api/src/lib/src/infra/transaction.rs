use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Expense,
    Income,
    Payment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub user_email: String,
    pub card_id: String,
    pub category_id: String,
    pub amount: i64, // in cents
    pub description: String,
    pub transaction_type: TransactionType,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransaction {
    pub card_id: String,
    pub category_id: String,
    pub amount: i64,
    pub description: String,
    pub transaction_type: TransactionType,
    pub date: DateTime<Utc>,
}
