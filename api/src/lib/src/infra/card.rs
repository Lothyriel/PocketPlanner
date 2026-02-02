use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    Credit,
    Debit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: String,
    pub user_email: String,
    pub name: String,
    pub card_type: CardType,
    pub credit_limit: Option<i64>,
    pub current_balance: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCard {
    pub name: String,
    pub card_type: CardType,
    pub credit_limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCard {
    pub name: Option<String>,
    pub credit_limit: Option<i64>,
}
