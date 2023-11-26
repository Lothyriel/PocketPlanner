use axum::{extract::Query, Json};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use rust_decimal_macros::dec;

#[derive(serde::Deserialize)]
pub struct Params {
    installment_value: Decimal,
    installments: u16,
}

#[derive(serde::Serialize)]
pub struct InstallmentModel {
    savings: Decimal,
    nubank: Decimal,
    picpay: Decimal,
}

#[axum_macros::debug_handler]
pub async fn handler(params: Query<Params>) -> Json<InstallmentModel> {
    todo!()
}

fn calculo(
    capital_inicial: Decimal,
    meses: u16,
    valor_mensal: Decimal,
    produto: String,
) -> Decimal {
    const valorSelic: f32 = 12.25;
    const ValorTesouro2027: f32 = valorSelic + 0.1736;
    const valorCDI: f32 = valorSelic - 0.1;
    let mut porcentagemCDIProduto = 100.;
    let mut taxaMensal = 0.0;
    let mut tempoFinal = 1;
    let mut montante = Decimal::ZERO;
    let mut valorMensalAcumulado = valor_mensal * Decimal::from_u16(meses).unwrap();
    let mut valorTotalInvestido = capital_inicial + valorMensalAcumulado;

    if (produto == "poupanca") {
        if (valorSelic > 8.5) {
            taxaMensal = 0.5;
        } else {
            taxaMensal = valorSelic * 70. / 100. / 12.;
        }
    } else if (produto == "nubank") {
        porcentagemCDIProduto = 100.;
        taxaMensal = (valorCDI * porcentagemCDIProduto) / 100. / 12.;
    } else if (produto == "picpay") {
        porcentagemCDIProduto = 102.0;
        taxaMensal = (valorCDI * porcentagemCDIProduto) / 100. / 12.;
    } else if (produto == "tesouro") {
        taxaMensal = ValorTesouro2027 / 12.;
    } else if (produto == "selic") {
        taxaMensal = ValorTesouro2027 / 12.;
    } else if (produto == "fgts") {
        taxaMensal = 3. / 12.;
    } else {
        taxaMensal = (valorSelic * 70.) / 100. / 12.;
    }
    taxaMensal /= 100.;

    while tempoFinal <= meses {
        montante = capital_inicial * (dec!(1) + Decimal::from_f32(taxaMensal).unwrap());
        capital_inicial = montante + valor_mensal;
        tempoFinal += 1;
    }
    montante += valor_mensal;

    let produto_bruto = montante;
    let produto_ir = get_ir(valorTotalInvestido, produto_bruto, meses);
    let produto_liquido = produto_bruto - produto_ir;

    todo!()
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
