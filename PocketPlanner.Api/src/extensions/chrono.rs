use std::str::FromStr;

use anyhow::{anyhow, Error};
use chrono::NaiveDate;

pub trait NaiveDateExt {
    fn from_str_pt(date: &str, separator: char) -> Result<NaiveDate, Error>;
}

impl NaiveDateExt for NaiveDate {
    fn from_str_pt(date: &str, separator: char) -> Result<NaiveDate, Error> {
        let segments: Vec<_> = date.split(separator).collect();

        if segments.len() != 3 {
            return Err(anyhow!("Invalid date {}", date));
        }

        let date = format!("{}-{}-{}", segments[2], segments[1], segments[0]);

        Ok(NaiveDate::from_str(&date)?)
    }
}
