use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use rust_decimal::Decimal;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub value: Decimal,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub description: String,
}
