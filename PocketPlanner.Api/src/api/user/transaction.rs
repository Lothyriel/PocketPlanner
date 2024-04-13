use axum::{extract::State, routing, Extension, Json, Router};
use rust_decimal::Decimal;

use crate::{
    api::{auth::UserClaims, AppState, ResponseResult},
    application::model::transaction::Transaction,
};

#[derive(serde::Deserialize)]
struct Params {
    value: Decimal,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(get))
        .route("/", routing::post(add))
        .with_state(state)
}

async fn get(
    State(state): State<AppState>,
    Extension(user_claims): Extension<UserClaims>,
) -> ResponseResult<Vec<Transaction>> {
    let extract = state.transactions.get_extract(user_claims.email).await?;

    Ok(Json(extract))
}

async fn add(
    State(state): State<AppState>,
    Extension(user_claims): Extension<UserClaims>,
    Json(params): Json<Params>,
) -> ResponseResult<()> {
    let tx = Transaction::new(user_claims.email, params.value);

    state.transactions.insert(tx).await?;

    Ok(Json(()))
}

async fn delete() {}
