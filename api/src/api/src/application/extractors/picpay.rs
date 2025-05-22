use std::io::Read;

use chrono::NaiveDate;
use lopdf::Document;

use crate::{
    application::{extractors::ParsingError, model::credit_card::CreditCardEntry},
    extensions::chrono::NaiveDateExt,
};

use super::{CreditCardInvoiceExtractor, ExtractError};

pub struct Picpay;

const HEADER_PAGES_COUNT: u32 = 2;
const FOOTER_PAGES_COUNT: u32 = 2;

impl CreditCardInvoiceExtractor for Picpay {
    fn extract_entries(
        data: impl Read,
        expected: NaiveDate,
    ) -> Result<Vec<CreditCardEntry>, ExtractError> {
        let document = Document::load_from(data)?;

        validate_invoice_date(&document, expected)?;

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
                let date = format!("{}/{}", chunk[0], 1);
                CreditCardEntry::try_from_chunk(&date, &chunk[1], &chunk[2])
            })
            .collect();

        Ok(entries)
    }
}

fn validate_invoice_date(document: &Document, expected: NaiveDate) -> Result<(), ExtractError> {
    const IGNORE_LINES_FIRST_PAGE_COUNT: usize = 3;

    let first_page = document.extract_text(&[1]).unwrap_or_default();

    let date_text = first_page
        .split('\n')
        .nth(IGNORE_LINES_FIRST_PAGE_COUNT)
        .ok_or(ParsingError::MissingData)?;

    let matches = [' ', '|'];
    let due_date = NaiveDate::from_str_pt(date_text.trim_matches(matches.as_slice()), '-')?;

    if due_date == expected {
        return Err(ExtractError::InvalidInvoiceDate {
            got: due_date,
            expected,
        });
    }

    Ok(())
}
