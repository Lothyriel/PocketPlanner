use axum::{extract::Query, Json};
use rust_decimal::Decimal;

#[derive(serde::Deserialize)]
pub struct Params {
    installment_value: Decimal,
    installments: u16,
}

#[derive(serde::Serialize)]
pub struct InstallmentsModel {}

#[axum_macros::debug_handler]
pub async fn handler(params: Query<Params>) -> Json<InstallmentsModel> {
    todo!()
}
