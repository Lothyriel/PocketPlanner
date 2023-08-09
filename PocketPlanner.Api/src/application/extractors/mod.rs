use std::io::Read;

use anyhow::Error;

use super::model::credit_card::CreditCardEntry;

pub mod picpay;

pub trait CreditCardExtractor {
    fn extract_entries(data: impl Read, month: u32, year: u32) -> Result<Vec<CreditCardEntry>, Error>;
}
