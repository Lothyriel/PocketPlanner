use surrealdb::sql::Thing;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Transaction {
    // I really wanted to serialize this as String (need to make serde::with work)
    id: Thing,
    amount: usize,
    description: String,
}
