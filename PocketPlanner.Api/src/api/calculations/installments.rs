use axum::{extract::Query, Json};
use rust_decimal::Decimal;

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

// function calculo(capitalInicial: number, meses: number, valorMensal: number) {
//   const valorSelic = 12.25
//   const ValorTesouro2027 = valorSelic + 0.1736
//   const valorCDI = valorSelic - 0.1
//   let porcentagemCDIProduto = 100
//   let taxaMensal = 0
//   let tempoFinal = 1
//   let montante = 0
//   let valorMensalAcumulado = valorMensal * meses
//   let valorTotalInvestido = capitalInicial + valorMensalAcumulado
//
//   if (params.produto === 'poupanca') {
//     if (valorSelic > 8.5) {
//       taxaMensal = 0.5
//     } else {
//       taxaMensal = (valorSelic * 70) / 100 / 12
//     }
//   } else if (params.produto === 'nubank') {
//     porcentagemCDIProduto = 100;
//     taxaMensal = (valorCDI * porcentagemCDIProduto) / 100 / 12;
//   } else if (params.produto === 'picpay') {
//     porcentagemCDIProduto = 102
//     taxaMensal = (valorCDI * porcentagemCDIProduto) / 100 / 12;
//   } else if (params.produto === 'tesouro') {
//     taxaMensal = ValorTesouro2027 / 12
//   } else if (params.produto === 'selic') {
//     taxaMensal = ValorTesouro2027 / 12
//   } else if (params.produto === 'fgts') {
//     taxaMensal = 3 / 12;
//   } else {
//     taxaMensal = (valorSelic * 70) / 100 / 12
//   }
//   taxaMensal /= 100
//
//   while (tempoFinal <= meses) {
//     montante = capitalInicial * (1 + taxaMensal)
//     capitalInicial = montante + valorMensal
//     tempoFinal += 1
//   }
//   montante += valorMensal
//
//   let produtoBruto = montante
//   let produtoIr = calcula_ir(valorTotalInvestido, produtoBruto, meses)
//   let produtoLiquido = produtoBruto - produtoIr
// }
//
// function calcula_ir(valorInvestido: number, valorBruto: number, qtdMeses: number) {
//   let rendimento = valorBruto - valorInvestido
//   let ir = 0
//
//   if (qtdMeses <= 6) {
//     ir = rendimento * 22.5
//   } else if (qtdMeses > 6 && qtdMeses <= 12) {
//     ir = rendimento * 20
//   } else if (qtdMeses > 12 && qtdMeses < 24) {
//     ir = rendimento * 17.5
//   } else if (qtdMeses >= 24) {
//     ir = rendimento * 15
//   }
//
//   return ir / 100
// }
