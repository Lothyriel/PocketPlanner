use std::io::Read;

use anyhow::Error;
use lopdf::Document;

use crate::application::model::credit_card::CreditCardEntry;

use super::CreditCardExtractor;

pub struct Picpay;

impl CreditCardExtractor for Picpay {
    fn extract_entries(data: impl Read, year: i32) -> Result<Vec<CreditCardEntry>, Error> {
        let document = Document::load_from(data)?;

        const HEADER_PAGES_COUNT: u32 = 2;
        const FOOTER_PAGES_COUNT: u32 = 2;

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
