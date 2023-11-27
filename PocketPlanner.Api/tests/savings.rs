#[cfg(test)]
mod tests {
    use pocket_planner::api::calculations::savings;
    use rust_decimal_macros::dec;

    #[test]
    fn savings() {
        let s = savings::get_revenue(&savings::Params {
            initial: dec!(1000),
            contribution: dec!(0),
            months: 10,
        });

        assert_eq!(s.picpay, dec!(1086.5673405142189276291334422));
    }
}
