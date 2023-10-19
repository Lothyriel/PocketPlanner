#[cfg(test)]
mod tests {
    use std::{fs::File, str::FromStr};

    use chrono::{Datelike, NaiveDate};
    use rust_decimal::Decimal;

    use pocket_planner::application::extractors::{picpay::Picpay, CreditCardInvoiceFileExtractor};

    #[test]
    fn picpay_extraction() {
        let file = File::open("tests/data/picpay_fatura_teste.pdf").unwrap();

        let date = NaiveDate::from_ymd_opt(2023, 8, 1).unwrap();

        let info = Picpay::extract_entries(file, date).unwrap();

        assert_eq!(info.len(), 2);

        let globo = &info[0];
        assert_eq!((globo.date.day(), globo.date.month()), (23, 4));
        assert_eq!(globo.value, Decimal::from_str("29.90").unwrap());

        let amazon = &info[1];
        assert_eq!((amazon.date.day(), amazon.date.month()), (19, 7));
        assert_eq!(amazon.value, Decimal::from_str("14.90").unwrap());
    }
}
