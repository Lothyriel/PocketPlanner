use axum::{extract::Query, Json};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Params {
    financed_value: Decimal,
    installments: u16,
    upfront_value: Decimal,
}

const CDI: Decimal = dec!(12.35);

#[derive(serde::Serialize)]
pub struct InstallmentsModel {
    params: Params,
    savings: Decimal,
}

#[axum_macros::debug_handler]
pub async fn handler(Query(params): Query<Params>) -> Json<InstallmentsModel> {
    let discount = params.financed_value - params.upfront_value;
    let installment_value = params.financed_value / Decimal::from(params.installments);

    let mut remaining = params.financed_value;
    let mut earnings = Decimal::ZERO;

    for _ in 0..params.installments {
        earnings += remaining * (CDI / dec!(12) / dec!(100));
        remaining -= installment_value;
    }

    Json(InstallmentsModel {
        savings: earnings - discount,
        params,
    })
}
