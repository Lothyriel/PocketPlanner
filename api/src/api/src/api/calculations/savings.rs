use axum::{extract::Query, Json};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use rust_decimal_macros::dec;

use super::{CDI, SELIC};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Params {
    pub initial: Decimal,
    pub contribution: Decimal,
    pub months: u16,
}

#[derive(serde::Serialize)]
pub struct SavingsModel {
    revenue: Revenue,
    params: Params,
}

#[derive(serde::Serialize, Debug)]
pub struct Revenue {
    pub savings: Decimal,
    pub nubank: Decimal,
    pub picpay: Decimal,
    pub treasury: Decimal,
    pub selic: Decimal,
    pub fgts: Decimal,
}

struct SavingSimulation {
    total_invested: Decimal,
    savings: Decimal,
}

const TREASURY_2027: f64 = SELIC + 0.1736;
const SAVINGS_RATE: f64 = if SELIC > 8.5 {
    0.5
} else {
    SELIC * 70. / 100. / 12.
};

pub async fn handler(Query(params): Query<Params>) -> Json<SavingsModel> {
    let revenue = get_revenue(&params);

    Json(SavingsModel { params, revenue })
}

pub fn get_revenue(params: &Params) -> Revenue {
    let c = |monthly_fee| {
        let s = get_gross(params, monthly_fee);
        s.savings - get_ir(s.total_invested, s.savings, params.months)
    };

    Revenue {
        savings: c(SAVINGS_RATE),
        nubank: c(CDI * 100. / 100. / 12.),
        picpay: c(CDI * 102. / 100. / 12.),
        treasury: c(TREASURY_2027 / 12.),
        selic: c(TREASURY_2027 / 12.),
        fgts: get_gross(params, 3. / 12.).savings,
    }
}

fn get_gross(params: &Params, monthly_rate: f64) -> SavingSimulation {
    let monthly_rate = Decimal::from_f64(monthly_rate).unwrap();

    let mut savings = params.initial;

    for _ in 0..params.months {
        savings *= dec!(1) + monthly_rate / dec!(100);
        savings += params.contribution;
    }

    savings += params.contribution;

    SavingSimulation {
        savings,
        total_invested: {
            let initial = params.initial;
            let contribution = params.contribution;
            initial + contribution * Decimal::from_u16(params.months).unwrap()
        },
    }
}

fn get_ir(invested: Decimal, gross: Decimal, months: u16) -> Decimal {
    let ir_fee = match months {
        0..=6 => dec!(22.5),
        7..=12 => dec!(20.0),
        13..=23 => dec!(17.5),
        24.. => dec!(15.0),
    };

    let revenue = gross - invested;

    ir_fee * revenue / dec!(100)
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    #[test]
    fn savings() {
        let s = super::get_revenue(&super::Params {
            initial: dec!(1000),
            contribution: dec!(0),
            months: 10,
        });

        assert_eq!(s.picpay, dec!(1086.5673405142189276291334422));
    }
}
