use rust_decimal::Decimal;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    email: String,
    value: Decimal,
}
