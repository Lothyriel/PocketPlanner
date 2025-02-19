use axum::{extract::Query, Json};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Params {
    financed: Decimal,
    installments: u16,
    upfront: Decimal,
}

// use api and cache it for X hours
const CDI: Decimal = dec!(12.35);

#[derive(serde::Serialize)]
pub struct InstallmentsModel {
    params: Params,
    savings: Decimal,
}

#[axum_macros::debug_handler]
pub async fn handler(Query(params): Query<Params>) -> Json<InstallmentsModel> {
    let discount = params.financed - params.upfront;
    let installment_value = params.financed / Decimal::from(params.installments);

    let monthly_rate = CDI / dec!(12) / dec!(100);

    let earnings = (0..params.installments)
        .fold((Decimal::ZERO, params.financed), |(e, r), _| {
            let earnings = e + r * monthly_rate;
            let remaining = r - installment_value;

            (earnings, remaining)
        })
        .0;

    Json(InstallmentsModel {
        savings: earnings - discount,
        params,
    })
}
