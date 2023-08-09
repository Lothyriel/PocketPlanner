use std::io::Read;

use anyhow::{anyhow, Error};
use chrono::{Datelike, NaiveDate};
use lopdf::Document;

use crate::{application::model::credit_card::CreditCardEntry, extensions::chrono::NaiveDateExt};

use super::CreditCardExtractor;

pub struct Picpay;

impl CreditCardExtractor for Picpay {
    fn extract_entries(
        data: impl Read,
        month: u32,
        year: u32,
    ) -> Result<Vec<CreditCardEntry>, Error> {
        const HEADER_PAGES_COUNT: u32 = 2;
        const FOOTER_PAGES_COUNT: u32 = 2;

        let document = Document::load_from(data)?;

        validate_invoice_date(&document, month, year)?;

        let pages_number = document
            .get_pages()
            .iter()
            .last()
            .map(|p| *p.0)
            .unwrap_or_default();

        let lines: Vec<_> = (HEADER_PAGES_COUNT..pages_number - FOOTER_PAGES_COUNT)
            .map(|i| document.extract_text(&[i + 1]).unwrap_or_default())
            .flat_map(|p| {
                p.split('\n')
                    .skip_while(|&line| line != "Valor")
                    .skip(1)
                    .skip_while(|&line| line != "Valor")
                    .skip(1)
                    .map(|line| line.to_owned())
                    .collect::<Vec<_>>()
            })
            .collect();

        let entries = lines
            .chunks_exact(3)
            .flat_map(|chunk| {
                let date = format!("{}/{}", chunk[0], year);
                CreditCardEntry::try_from_chunk(&date, &chunk[1], &chunk[2])
            })
            .collect();

        Ok(entries)
    }
}

fn validate_invoice_date(document: &Document, month: u32, year: u32) -> Result<(), Error> {
    const IGNORE_LINES_FIRST_PAGE_COUNT: usize = 3;

    let first_page = document.extract_text(&[1]).unwrap_or_default();

    let date_text = first_page
        .split('\n')
        .skip(IGNORE_LINES_FIRST_PAGE_COUNT)
        .next()
        .ok_or_else(|| anyhow!("Data not in expected form"))?;

    let matches: &[_] = &[' ', '|'];
    let due_date = NaiveDate::from_str_pt(date_text.trim_matches(matches), '-')?;

    if due_date.month() != month || due_date.year() != year as i32 {
        return Err(anyhow!("This invoice is from {}/{}", month, year));
    }

    Ok(())
}
