use anyhow::Result;
use askama_web::WebTemplate;

use axum::{extract::State, response::IntoResponse, routing, Form, Router};

use crate::{AppError, AppState, Db};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(view))
        .route("/add", routing::post(action))
        .with_state(state)
}

pub async fn view(State(state): State<AppState>) -> Result<View, AppError> {
    let transactions = get_transactions(&state.db).await?;

    Ok(View { transactions })
}

async fn action(
    State(state): State<AppState>,
    Form(tx): Form<Transaction>,
) -> Result<impl IntoResponse, AppError> {
    add_transaction(&state.db, &tx).await?;

    Ok(Action { tx })
}

async fn get_transactions(db: &Db) -> Result<Vec<Transaction>> {
    let transactions = db
        .query("SELECT amount, description FROM transactions")
        .await?
        .take(0)?;

    Ok(transactions)
}

async fn add_transaction(conn: &Db, transaction: &Transaction) -> Result<()> {
    conn.query("INSERT INTO transactions (amount, description) VALUES ($amount, $description)")
        .bind(("amount", transaction.amount))
        .bind(("description", transaction.description.clone()))
        .await?;

    Ok(())
}

#[derive(serde::Deserialize)]
struct Transaction {
    pub amount: u64,
    pub description: String,
}

#[derive(askama::Template, WebTemplate)]
#[template(path = "transaction/view.html")]
pub struct View {
    transactions: Vec<Transaction>,
}

#[derive(askama::Template, WebTemplate)]
#[template(path = "transaction/action.html")]
struct Action {
    tx: Transaction,
}
