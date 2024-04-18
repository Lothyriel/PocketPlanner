use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub email: String,
    pub value: Decimal,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub description: String,
}
