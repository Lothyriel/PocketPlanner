use axum::{extract::State, http::StatusCode, response::IntoResponse, routing, Extension, Router};

use crate::{
    infra::{transaction::Transaction, Db, DbState, UserClaims},
    AppResult, Json, Response,
};

pub fn router(state: DbState) -> Router {
    Router::new()
        .route("/", routing::get(get))
        .route("/", routing::post(create))
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
    Json(tx): Json<CreateTransaction>,
) -> AppResult<impl IntoResponse> {
    let db = state.db(&claims.email).await?;

    let tx = add_transaction(db, tx).await?;

    Ok((StatusCode::CREATED, Json(tx)))
}

async fn get_transactions(db: &Db) -> AppResult<Vec<Transaction>> {
    let transactions = db
        .query("SELECT amount, description FROM transactions")
        .await?
        .take(0)?;

    Ok(transactions)
}

async fn add_transaction(conn: &Db, tx: CreateTransaction) -> AppResult<Transaction> {
    let result: Option<Transaction> = conn
        .query("INSERT INTO transactions (amount, description) VALUES ($amount, $description)")
        .bind(("amount", tx.amount))
        .bind(("description", tx.description))
        .await?
        .take(0)?;

    Ok(result.expect("Expected to add"))
}

#[derive(serde::Deserialize)]
pub struct CreateTransaction {
    pub amount: u64,
    pub description: String,
}
