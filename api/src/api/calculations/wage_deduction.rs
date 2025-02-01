use axum::{extract::Query, Json};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(serde::Deserialize)]
pub struct Params {
    wage: Decimal,
    dependents: Option<Decimal>,
}

#[derive(serde::Serialize)]
pub struct WageDeductionsModel {
    inss: Decimal,
    inss_percent: Decimal,
    irrf: Decimal,
    irrf_percent: Decimal,
    total_deductions: Decimal,
    gross: Decimal,
    net: Decimal,
    aliquot_irrf: Decimal,
}

pub async fn handler(params: Query<Params>) -> Json<WageDeductionsModel> {
    let deductions = get_deductions(params.wage, params.dependents.unwrap_or_default());

    Json(deductions)
}

fn get_deductions(gross_wage: Decimal, dependents: Decimal) -> WageDeductionsModel {
    let ranges = get_irrf_ranges();
    let dependent_discount = dec!(189.59);

    let a = get_inss(gross_wage);

    let r = a.inss;
    let n = a.percent;
    let o = ranges
        .iter()
        .find(|x| x.min < gross_wage - r && (x.max > gross_wage - r))
        .expect("Expected at least one value in range");

    let c = gross_wage - r - dependents * dependent_discount;
    let l = o.percent / dec!(100);
    let i = o.parcel_tax;
    let u = Decimal::max(c * l - i, dec!(0));
    let s = r + u;
    let d = (u / c) * dec!(100);

    WageDeductionsModel {
        inss: r,
        inss_percent: n,
        irrf: u,
        irrf_percent: l,
        total_deductions: s,
        gross: gross_wage,
        net: gross_wage - s,
        aliquot_irrf: d,
    }
}

fn get_inss(gross_wage: Decimal) -> InssDeduction {
    let ranges = get_inss_ranges();

    let hundred = dec!(100);

    let a = ranges[0].max * (ranges[0].percent / hundred);
    let r = (ranges[1].max - ranges[0].max) * (ranges[1].percent / hundred);
    let n = (ranges[2].max - ranges[1].max) * (ranges[2].percent / hundred);

    let (range, inss) = ranges
        .iter()
        .enumerate()
        .find(|(_, x)| x.min <= gross_wage && gross_wage <= x.max)
        .expect("Expected to find at least one value in range");

    let inss_deduction = dec!(0.14) * ranges[3].max - dec!(174.08);

    let o = match range {
        1 => (inss.percent / hundred) * gross_wage,
        2 => a + (gross_wage - ranges[0].max) * (ranges[1].percent / hundred),
        3 => a + r + (gross_wage - ranges[1].max) * (ranges[2].percent / hundred),
        4 => a + r + n + (gross_wage - ranges[2].max) * (ranges[3].percent / hundred),
        5 => inss_deduction,
        _ => 0.into(),
    };

    let c = if o != inss_deduction {
        o / gross_wage
    } else {
        0.into()
    };

    InssDeduction {
        inss: o,
        percent: c,
    }
}

struct InssDeduction {
    inss: Decimal,
    percent: Decimal,
}

struct InssRange {
    percent: Decimal,
    min: Decimal,
    max: Decimal,
}

struct IrrfRange {
    percent: Decimal,
    min: Decimal,
    max: Decimal,
    parcel_tax: Decimal,
}

fn get_inss_ranges() -> [InssRange; 5] {
    [
        InssRange {
            percent: dec!(7.5),
            min: 0.into(),
            max: 1320.into(),
        },
        InssRange {
            percent: 9.into(),
            min: dec!(1320.01),
            max: dec!(2571.29),
        },
        InssRange {
            percent: 12.into(),
            min: dec!(2571.3),
            max: dec!(3856.94),
        },
        InssRange {
            percent: 14.into(),
            min: dec!(3856.95),
            max: dec!(7507.49),
        },
        InssRange {
            percent: 100.into(),
            min: dec!(7507.5),
            max: Decimal::MAX,
        },
    ]
}

fn get_irrf_ranges() -> [IrrfRange; 5] {
    [
        IrrfRange {
            percent: 0.into(),
            parcel_tax: 0.into(),
            min: 0.into(),
            max: dec!(2112),
        },
        IrrfRange {
            percent: dec!(7.5),
            parcel_tax: dec!(158.4),
            min: dec!(2112.01),
            max: dec!(2826.65),
        },
        IrrfRange {
            percent: 15.into(),
            parcel_tax: dec!(370.4),
            min: dec!(2826.66),
            max: dec!(3751.05),
        },
        IrrfRange {
            percent: dec!(22.5),
            parcel_tax: dec!(651.73),
            min: dec!(3751.06),
            max: dec!(4664.68),
        },
        IrrfRange {
            percent: dec!(27.5),
            parcel_tax: dec!(884.96),
            min: dec!(4664.69),
            max: Decimal::MAX,
        },
    ]
}
