use axum::{
    extract::{Path, State},
    routing, Extension, Json, Router,
};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use rust_decimal::Decimal;

use crate::{
    api::{auth::UserClaims, AppState, ResponseResult},
    application::model::transaction::Transaction,
};

#[derive(serde::Deserialize)]
struct Params {
    value: Decimal,
    description: String,
    tags: Vec<String>,
}

#[derive(serde::Serialize)]
struct Model {
    id: ObjectId,
    value: Decimal,
    date: DateTime<Utc>,
    description: String,
    tags: Vec<String>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(get))
        .route("/", routing::post(add))
        .route("/:id", routing::delete(delete))
        .with_state(state)
}

async fn get(
    State(state): State<AppState>,
    Extension(user_claims): Extension<UserClaims>,
) -> ResponseResult<Vec<Model>> {
    let extract = state.transactions.get_extract(&user_claims.email).await?;

    let response = extract
        .into_iter()
        .map(|t| Model {
            id: t.id,
            value: t.value,
            date: t.date,
            description: t.description,
            tags: t.tags,
        })
        .collect();

    Ok(Json(response))
}

async fn add(
    State(state): State<AppState>,
    Extension(user_claims): Extension<UserClaims>,
    Json(params): Json<Params>,
) -> ResponseResult<ObjectId> {
    let tx = Transaction {
        id: ObjectId::new(),
        date: chrono::Utc::now(),
        email: user_claims.email,
        value: params.value,
        tags: params.tags,
        description: params.description,
    };

    let id = state.transactions.insert(&tx).await?;

    Ok(Json(id))
}

async fn delete(
    State(state): State<AppState>,
    Extension(user_claims): Extension<UserClaims>,
    Path(id): Path<ObjectId>,
) -> ResponseResult<()> {
    state.transactions.delete(&user_claims.email, id).await?;

    Ok(Json(()))
}
