#[cfg(test)]
mod tests {
    use std::{fs::File, str::FromStr};

    use anyhow::Error;
    use chrono::Datelike;
    use rust_decimal::Decimal;

    use pocket_planner::application::extractors::{picpay::Picpay, CreditCardExtractor};

    #[test]
    fn picpay_extraction() -> Result<(), Error> {
        let file = File::open("tests/data/picpay_fatura_teste.pdf")?;

        let info = Picpay::extract_entries(file, 08, 2023)?;

        assert_eq!(info.len(), 2);

        let globo = &info[0];
        assert_eq!((globo.date.day(), globo.date.month()), (23, 04));
        assert_eq!(globo.value, Decimal::from_str("29.90")?);

        let amazon = &info[1];
        assert_eq!((amazon.date.day(), amazon.date.month()), (19, 07));
        assert_eq!(amazon.value, Decimal::from_str("14.90")?);

        Ok(())
    }
}
