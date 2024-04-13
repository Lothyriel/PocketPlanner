use rust_decimal::Decimal;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    email: String,
    value: Decimal,
}

impl Transaction {
    pub fn new(email: String, value: Decimal) -> Self {
        Self { email, value }
    }
}
