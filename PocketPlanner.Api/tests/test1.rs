#[cfg(test)]
mod tests {
    use anyhow::Error;

    #[test]
    fn it_works() -> Result<(), Error> {
        assert_eq!(4, 4);

        Ok(())
    }
}