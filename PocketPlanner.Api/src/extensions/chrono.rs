use std::str::FromStr;

use chrono::NaiveDate;

use crate::application::extractors::ParsingError;

pub trait NaiveDateExt {
    fn from_str_pt(date: &str, separator: char) -> Result<NaiveDate, ParsingError>;
}

impl NaiveDateExt for NaiveDate {
    fn from_str_pt(date: &str, separator: char) -> Result<NaiveDate, ParsingError> {
        let segments: Vec<_> = date.split(separator).collect();

        if segments.len() != 3 {
            return Err(ParsingError::MissingData);
        }

        let date = format!("{}-{}-{}", segments[2], segments[1], segments[0]);

        Ok(NaiveDate::from_str(&date)?)
    }
}
