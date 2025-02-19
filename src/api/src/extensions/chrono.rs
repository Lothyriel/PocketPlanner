use std::str::FromStr;

use chrono::NaiveDate;

use crate::application::extractors::ParsingError;

pub trait NaiveDateExt {
    fn from_str_pt(date: &str, separator: char) -> Result<NaiveDate, ParsingError>;
}

impl NaiveDateExt for NaiveDate {
    fn from_str_pt(date: &str, separator: char) -> Result<NaiveDate, ParsingError> {
        let mut segments = date.split(separator);

        let day = get_segment(segments.next())?;

        let month = get_segment(segments.next())?;

        let year = get_segment(segments.next())?;

        let date = format!("{}-{}-{}", year, month, day);

        Ok(NaiveDate::from_str(&date)?)
    }
}

fn get_segment(segment: Option<&str>) -> Result<&str, ParsingError> {
    segment.ok_or(ParsingError::MissingData)
}
