use std::str::FromStr;

use anyhow::{anyhow, Error};
use chrono::NaiveDate;
use rust_decimal::Decimal;

pub struct CreditCardEntry {
    pub date: NaiveDate,
    pub description: String,
    pub value: Decimal,
}

impl CreditCardEntry {
    pub fn try_from_chunk(date: &str, description: &str, value: &str) -> Result<Self, Error> {
        let segments: Vec<_> = date.split('/').collect();

        if segments.len() != 3 {
            return Err(anyhow!("Invalid date {}", date));
        }

        let date = format!("{}-{}-{}", segments[2], segments[1], segments[0]);

        Ok(Self {
            date: NaiveDate::from_str(&date)?,
            description: description.to_owned(),
            value: Decimal::from_str_exact(&value.replace(",", "."))?,
        })
    }
}
