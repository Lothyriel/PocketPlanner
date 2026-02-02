#![allow(unused)]

use std::io::Read;

use chrono::NaiveDate;

use super::model::credit_card::CreditCardEntry;

pub mod nubank;
pub mod picpay;

#[allow(dead_code)]
pub trait CreditCardInvoiceExtractor {
    fn extract_entries(
        data: impl Read,
        expected: NaiveDate,
    ) -> Result<Vec<CreditCardEntry>, ExtractError>;
}

#[derive(thiserror::Error, Debug)]
pub enum ExtractError {
    #[error("Expected invoice from {expected} got {got}")]
    InvalidInvoiceDate { expected: NaiveDate, got: NaiveDate },
    #[error("{0}")]
    Pdf(#[from] lopdf::Error),
    #[error("{0}")]
    Parsing(#[from] ParsingError),
}

#[derive(thiserror::Error, Debug)]
pub enum ParsingError {
    #[error("Data was misssing")]
    MissingData,
    #[error("{0}")]
    DecimalParsing(#[from] rust_decimal::Error),
    #[error("{0}")]
    DateParsing(#[from] chrono::format::ParseError),
}

#[cfg(test)]
mod tests {
    use std::{fs::File, str::FromStr};

    use chrono::{Datelike, NaiveDate};
    use rust_decimal::Decimal;

    use crate::application::extractors::{picpay::Picpay, CreditCardInvoiceExtractor};

    #[test]
    fn picpay_extraction() {
        let file = File::open("tests/data/picpay_fatura_teste.pdf").unwrap();

        let date = NaiveDate::from_ymd_opt(2023, 8, 1).unwrap();

        let info = Picpay::extract_entries(file, date).unwrap();

        assert_eq!(info.len(), 2);

        let globo = &info[0];
        assert_eq!((globo.date.day(), globo.date.month()), (23, 4));
        assert_eq!(globo.value, Decimal::from_str("29.90").unwrap());

        let amazon = &info[1];
        assert_eq!((amazon.date.day(), amazon.date.month()), (19, 7));
        assert_eq!(amazon.value, Decimal::from_str("14.90").unwrap());
    }
}
