use std::io::Read;

use chrono::NaiveDate;

use super::model::credit_card::CreditCardEntry;

pub mod nubank;
pub mod picpay;

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
