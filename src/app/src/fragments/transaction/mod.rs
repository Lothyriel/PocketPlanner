use anyhow::{anyhow, Result};
use askama::Template;

use lib::templates::{self, transaction::Transaction};

use crate::{connect_db, Form, FromFormData};

pub fn view() -> Result<String> {
    let mut conn = connect_db()?;

    Ok(templates::transaction::view(&mut conn)?.render()?)
}

pub fn action(form: Form) -> Result<String> {
    let mut conn = connect_db()?;
    let tx = FromFormData::from(form)?;

    Ok(templates::transaction::action(&mut conn, tx)?.render()?)
}

// macro?
impl FromFormData for Transaction {
    fn from(form: Form) -> Result<Self> {
        let amount = form
            .get("amount")
            .ok_or_else(|| anyhow!("No amount"))?
            .parse()?;

        let description = form
            .get("description")
            .ok_or_else(|| anyhow!("No description"))?
            .to_string();

        Ok(Transaction {
            amount,
            description,
        })
    }
}
