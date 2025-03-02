use anyhow::Result;
use rusqlite::{params, Connection};

pub fn view(conn: &mut Connection) -> Result<View> {
    let transactions = get_transactions(conn)?;

    Ok(View { transactions })
}

pub fn action(conn: &mut Connection, tx: Transaction) -> Result<Action> {
    add_transaction(conn, tx)?;

    Ok(Action)
}

fn get_transactions(conn: &mut Connection) -> Result<Vec<Transaction>> {
    let mut statement = conn.prepare("SELECT amount, description FROM transactions")?;

    let rows = statement.query_map([], |row| {
        Ok(Transaction {
            amount: row.get(0)?,
            description: row.get(1)?,
        })
    })?;

    let transactions = rows.collect::<Result<_, _>>()?;

    Ok(transactions)
}

fn add_transaction(conn: &mut Connection, transaction: Transaction) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute(
        "INSERT INTO transactions (amount, description) VALUES (?1, ?2)",
        params![transaction.amount, transaction.description],
    )?;

    tx.commit()?;

    Ok(())
}

#[derive(serde::Deserialize)]
pub struct Transaction {
    pub amount: u64,
    pub description: String,
}

#[derive(askama::Template)]
#[template(path = "transaction/view.html")]
pub struct View {
    transactions: Vec<Transaction>,
}

#[derive(askama::Template)]
#[template(path = "transaction/action.html")]
pub struct Action;
