use axum::{extract::Query, Json};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Params {
    financed: Decimal,
    installments: u16,
    upfront: Decimal,
    cdi: Decimal,
    tax: Decimal,
}

#[derive(serde::Serialize)]
pub struct InstallmentsModel {
    params: Params,
    savings: Decimal,
}

pub async fn handler(Query(params): Query<Params>) -> Json<InstallmentsModel> {
    let discount = params.financed - params.upfront;
    let installment_value = params.financed / Decimal::from(params.installments);

    let monthly_rate = (params.cdi * (dec!(1) - params.tax)) / dec!(12) / dec!(100);

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
