use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::{application::extractors::ParsingError, extensions::chrono::NaiveDateExt};

#[allow(dead_code)]
pub struct CreditCardEntry {
    pub date: NaiveDate,
    pub description: String,
    pub value: Decimal,
}

impl CreditCardEntry {
    pub fn try_from_chunk(
        date: &str,
        description: &str,
        value: &str,
    ) -> Result<Self, ParsingError> {
        Ok(Self {
            date: NaiveDate::from_str_pt(date, '/')?,
            description: description.to_owned(),
            value: Decimal::from_str_exact(&value.replace(',', "."))?,
        })
    }
}
