use axum::{
    Extension, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing,
};
use uuid::Uuid;

use crate::{
    AppError, AppResult, Json, Response,
    infra::{Card, CardType, CreateCard, DbState, UpdateCard, UserClaims},
};

pub fn router(state: DbState) -> Router {
    Router::new()
        .route("/", routing::get(list))
        .route("/", routing::post(create))
        .route("/{id}", routing::get(get))
        .route("/{id}", routing::put(update))
        .route("/{id}", routing::delete(delete))
        .with_state(state)
}

async fn list(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
) -> Response<Vec<Card>> {
    let email = claims.email;
    let cards = state
        .conn
        .call(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, user_email, name, card_type, credit_limit, current_balance
                 FROM cards WHERE user_email = ?1",
            )?;
            let cards = stmt
                .query_map([&email], |row| {
                    Ok(Card {
                        id: row.get(0)?,
                        user_email: row.get(1)?,
                        name: row.get(2)?,
                        card_type: parse_card_type(row.get::<_, String>(3)?),
                        credit_limit: row.get(4)?,
                        current_balance: row.get(5)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(cards)
        })
        .await?;

    Ok(Json(cards))
}

async fn get(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<String>,
) -> Response<Card> {
    let email = claims.email;
    let card = state
        .conn
        .call(move |conn| {
            Ok(conn.query_row(
                "SELECT id, user_email, name, card_type, credit_limit, current_balance
                 FROM cards WHERE id = ?1 AND user_email = ?2",
                [&id, &email],
                |row| {
                    Ok(Card {
                        id: row.get(0)?,
                        user_email: row.get(1)?,
                        name: row.get(2)?,
                        card_type: parse_card_type(row.get::<_, String>(3)?),
                        credit_limit: row.get(4)?,
                        current_balance: row.get(5)?,
                    })
                },
            )?)
        })
        .await?;

    Ok(Json(card))
}

async fn create(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
    Json(input): Json<CreateCard>,
) -> AppResult<impl IntoResponse> {
    let card = Card {
        id: Uuid::now_v7().to_string(),
        user_email: claims.email,
        name: input.name,
        card_type: input.card_type,
        credit_limit: input.credit_limit,
        current_balance: 0,
    };

    let card_clone = card.clone();
    state
        .conn
        .call(move |conn| {
            conn.execute(
                "INSERT INTO cards (id, user_email, name, card_type, credit_limit, current_balance)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (
                    &card_clone.id,
                    &card_clone.user_email,
                    &card_clone.name,
                    card_type_to_str(&card_clone.card_type),
                    &card_clone.credit_limit,
                    &card_clone.current_balance,
                ),
            )?;
            Ok(())
        })
        .await?;

    Ok((StatusCode::CREATED, Json(card)))
}

async fn update(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<String>,
    Json(input): Json<UpdateCard>,
) -> Response<Card> {
    let email = claims.email;
    let card = state
        .conn
        .call(move |conn| {
            // Update fields if provided
            if let Some(name) = &input.name {
                conn.execute(
                    "UPDATE cards SET name = ?1 WHERE id = ?2 AND user_email = ?3",
                    (&name, &id, &email),
                )?;
            }
            if let Some(credit_limit) = &input.credit_limit {
                conn.execute(
                    "UPDATE cards SET credit_limit = ?1 WHERE id = ?2 AND user_email = ?3",
                    (credit_limit, &id, &email),
                )?;
            }

            // Fetch updated card
            Ok(conn.query_row(
                "SELECT id, user_email, name, card_type, credit_limit, current_balance
                 FROM cards WHERE id = ?1 AND user_email = ?2",
                [&id, &email],
                |row| {
                    Ok(Card {
                        id: row.get(0)?,
                        user_email: row.get(1)?,
                        name: row.get(2)?,
                        card_type: parse_card_type(row.get::<_, String>(3)?),
                        credit_limit: row.get(4)?,
                        current_balance: row.get(5)?,
                    })
                },
            )?)
        })
        .await?;

    Ok(Json(card))
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
            // Delete related transactions first
            conn.execute(
                "DELETE FROM transactions WHERE card_id = ?1 AND user_email = ?2",
                [&id, &email],
            )?;

            // Delete the card
            let rows = conn.execute(
                "DELETE FROM cards WHERE id = ?1 AND user_email = ?2",
                [&id, &email],
            )?;
            Ok(rows > 0)
        })
        .await?;

    if !deleted {
        return Err(AppError::Validation("Card not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

fn parse_card_type(s: String) -> CardType {
    match s.as_str() {
        "credit" => CardType::Credit,
        "debit" => CardType::Debit,
        _ => CardType::Debit,
    }
}

fn card_type_to_str(t: &CardType) -> &'static str {
    match t {
        CardType::Credit => "credit",
        CardType::Debit => "debit",
    }
}
