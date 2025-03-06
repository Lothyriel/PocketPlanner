use anyhow::Result;
use rusqlite::{params, Connection};

use axum::{response::IntoResponse, routing, Form, Router};

use crate::connect_db;

use super::AppError;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(view))
        .route("/add", routing::post(action))
}

async fn view() -> Result<impl IntoResponse, AppError> {
    let mut conn = connect_db()?;

    let transactions = get_transactions(&mut conn)?;

    Ok(View { transactions })
}

async fn action(Form(tx): Form<Transaction>) -> Result<impl IntoResponse, AppError> {
    let mut conn = connect_db()?;

    add_transaction(&mut conn, &tx)?;

    Ok(Action { tx })
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

fn add_transaction(conn: &mut Connection, transaction: &Transaction) -> Result<()> {
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
pub struct Action {
    tx: Transaction,
}
