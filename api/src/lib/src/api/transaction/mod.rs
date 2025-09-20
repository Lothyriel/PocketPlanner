use axum::{extract::State, http::StatusCode, response::IntoResponse, routing, Extension, Router};

use crate::{
    infra::{transaction::Transaction, DbState, UserClaims},
    AppResult, Json, Response,
};

pub fn router(state: DbState) -> Router {
    Router::new()
        .route("/", routing::get(get))
        .route("/", routing::post(create))
        .with_state(state)
}

pub async fn get(
    State(_state): State<DbState>,
    Extension(_claims): Extension<UserClaims>,
) -> Response<Vec<Transaction>> {
    let transactions = get_transactions().await?;

    Ok(Json(transactions))
}

async fn create(
    State(_state): State<DbState>,
    Extension(_claims): Extension<UserClaims>,
    Json(tx): Json<CreateTransaction>,
) -> AppResult<impl IntoResponse> {
    let tx = add_transaction(tx).await?;

    Ok((StatusCode::CREATED, Json(tx)))
}

async fn get_transactions() -> AppResult<Vec<Transaction>> {
    todo!("SELECT amount, description FROM transactions")
}

async fn add_transaction(_tx: CreateTransaction) -> AppResult<Transaction> {
    todo!("INSERT INTO transactions (amount, description) VALUES ($amount, $description)")
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateTransaction {
    pub amount: u64,
    pub description: String,
}
