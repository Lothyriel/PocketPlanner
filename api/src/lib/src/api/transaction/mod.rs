use axum::{
    Extension, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppError, AppResult, Json, Response,
    infra::{CreateTransaction, DbState, Transaction, TransactionType, UserClaims},
};

pub fn router(state: DbState) -> Router {
    Router::new()
        .route("/", routing::get(list))
        .route("/", routing::post(create))
        .route("/{id}", routing::get(get))
        .route("/{id}", routing::delete(delete))
        .with_state(state)
}

#[derive(Deserialize)]
struct TransactionListQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}

async fn list(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
    Query(query): Query<TransactionListQuery>,
) -> Response<Vec<Transaction>> {
    let limit = query.limit.unwrap_or(50).clamp(1, 200) as i64;
    let offset = query.offset.unwrap_or(0) as i64;
    let email = claims.email;
    let transactions = state
        .conn
        .call(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, user_email, card_id, category_id, amount, description, transaction_type, date
                 FROM transactions WHERE user_email = ?1 ORDER BY date DESC LIMIT ?2 OFFSET ?3",
            )?;
            let transactions = stmt
                .query_map((&email, limit, offset), |row| {
                    Ok(Transaction {
                        id: row.get(0)?,
                        user_email: row.get(1)?,
                        card_id: row.get(2)?,
                        category_id: row.get(3)?,
                        amount: row.get(4)?,
                        description: row.get(5)?,
                        transaction_type: parse_transaction_type(row.get::<_, String>(6)?),
                        date: parse_date(row.get::<_, String>(7)?),
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(transactions)
        })
        .await?;

    Ok(Json(transactions))
}

async fn get(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<String>,
) -> Response<Transaction> {
    let email = claims.email;
    let transaction = state
        .conn
        .call(move |conn| {
            conn.query_row(
                "SELECT id, user_email, card_id, category_id, amount, description, transaction_type, date
                 FROM transactions WHERE id = ?1 AND user_email = ?2",
                [&id, &email],
                |row| {
                    Ok(Transaction {
                        id: row.get(0)?,
                        user_email: row.get(1)?,
                        card_id: row.get(2)?,
                        category_id: row.get(3)?,
                        amount: row.get(4)?,
                        description: row.get(5)?,
                        transaction_type: parse_transaction_type(row.get::<_, String>(6)?),
                        date: parse_date(row.get::<_, String>(7)?),
                    })
                },
            )
        })
        .await?;

    Ok(Json(transaction))
}

async fn create(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
    Json(input): Json<CreateTransaction>,
) -> AppResult<impl IntoResponse> {
    let transaction = Transaction {
        id: Uuid::now_v7().to_string(),
        user_email: claims.email.clone(),
        card_id: input.card_id.clone(),
        category_id: input.category_id,
        amount: input.amount,
        description: input.description,
        transaction_type: input.transaction_type.clone(),
        date: input.date,
    };

    let tx_clone = transaction.clone();
    let email = claims.email;
    let card_id = input.card_id;
    let amount = input.amount;
    let _tx_type = input.transaction_type;

    state
        .conn
        .call(move |conn| {
            // Verify the card belongs to the user
            let card_exists: bool = conn.query_row(
                "SELECT 1 FROM cards WHERE id = ?1 AND user_email = ?2",
                [&card_id, &email],
                |_| Ok(true),
            ).unwrap_or(false);

            if !card_exists {
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }

            // Insert transaction
            conn.execute(
                "INSERT INTO transactions (id, user_email, card_id, category_id, amount, description, transaction_type, date)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                (
                    &tx_clone.id,
                    &tx_clone.user_email,
                    &tx_clone.card_id,
                    &tx_clone.category_id,
                    &tx_clone.amount,
                    &tx_clone.description,
                    transaction_type_to_str(&tx_clone.transaction_type),
                    &tx_clone.date.to_rfc3339(),
                ),
            )?;

            // Update card balance
            let balance_change = amount;

            conn.execute(
                "UPDATE cards SET current_balance = current_balance + ?1 WHERE id = ?2",
                (balance_change, &card_id),
            )?;

            Ok(())
        })
        .await?;

    Ok((StatusCode::CREATED, Json(transaction)))
}

async fn delete(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let email = claims.email;
    let deleted = state
        .conn
        .call(move |conn| {
            // Get transaction details to reverse balance
            let (card_id, amount): (String, i64) = conn.query_row(
                "SELECT card_id, amount FROM transactions WHERE id = ?1 AND user_email = ?2",
                [&id, &email],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )?;

            // Reverse balance change
            let balance_change = -amount;

            conn.execute(
                "UPDATE cards SET current_balance = current_balance + ?1 WHERE id = ?2",
                (balance_change, &card_id),
            )?;

            // Delete transaction
            let rows = conn.execute(
                "DELETE FROM transactions WHERE id = ?1 AND user_email = ?2",
                [&id, &email],
            )?;

            Ok(rows > 0)
        })
        .await?;

    if !deleted {
        return Err(AppError::Validation("Transaction not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

fn parse_transaction_type(s: String) -> TransactionType {
    match s.as_str() {
        "expense" => TransactionType::Expense,
        "income" => TransactionType::Income,
        "payment" => TransactionType::Payment,
        _ => TransactionType::Expense,
    }
}

fn transaction_type_to_str(t: &TransactionType) -> &'static str {
    match t {
        TransactionType::Expense => "expense",
        TransactionType::Income => "income",
        TransactionType::Payment => "payment",
    }
}

fn parse_date(s: String) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}
