use axum::{extract::State, routing, Extension, Form, Router};

use crate::{
    infra::{Db, DbState, UserClaims},
    AppResult, Json, Response,
};

pub fn router(state: DbState) -> Router {
    Router::new()
        .route("/", routing::get(get))
        .route("/add", routing::post(create))
        .with_state(state)
}

pub async fn get(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
) -> Response<Vec<Transaction>> {
    let db = state.db(&claims.email).await?;
    let transactions = get_transactions(db).await?;

    Ok(Json(transactions))
}

async fn create(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
    Form(tx): Form<Transaction>,
) -> AppResult<()> {
    let db = state.db(&claims.email).await?;

    add_transaction(db, &tx).await?;

    Ok(())
}

async fn get_transactions(db: &Db) -> AppResult<Vec<Transaction>> {
    let transactions = db
        .query("SELECT amount, description FROM transactions")
        .await?
        .take(0)?;

    Ok(transactions)
}

async fn add_transaction(conn: &Db, transaction: &Transaction) -> AppResult<()> {
    conn.query("INSERT INTO transactions (amount, description) VALUES ($amount, $description)")
        .bind(("amount", transaction.amount))
        .bind(("description", transaction.description.clone()))
        .await?;

    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Transaction {
    pub amount: u64,
    pub description: String,
}
