#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use anyhow::Error;
    use lopdf::Document;

    #[test]
    fn pdf_extraction() -> Result<(), Error> {
        let file = File::open("tests/data/fatura_teste.pdf")?;

        let info = extract_pdf_info(file)?;

        for a in info[0].split('\n') {
            println!("{a}");
        }

        assert_eq!(info.len(), 1);

        Ok(())
    }

    fn extract_pdf_info(data: impl Read) -> Result<Vec<String>, Error> {
        let document = Document::load_from(data)?;

        const HEADER_PAGES_COUNT: u32 = 2;
        const FOOTER_PAGES_COUNT: u32 = 2;

        let total_pages_number = document
            .get_pages()
            .iter()
            .last()
            .map(|p| *p.0)
            .unwrap_or_default();

        let extract_map = (HEADER_PAGES_COUNT..total_pages_number - FOOTER_PAGES_COUNT)
            .map(|i| document.extract_text(&[i + 1]).unwrap_or_default());

        Ok(extract_map.collect())
    }
}
