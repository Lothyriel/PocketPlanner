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
    infra::{Category, CreateCategory, DbState, UserClaims},
};

pub fn router(state: DbState) -> Router {
    Router::new()
        .route("/", routing::get(list))
        .route("/", routing::post(create))
        .route("/{id}", routing::delete(delete))
        .with_state(state)
}

async fn list(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
) -> Response<Vec<Category>> {
    let email = claims.email;
    let categories = state
        .conn
        .call(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, user_email, name, color
                 FROM categories
                 WHERE user_email IS NULL OR user_email = ?1",
            )?;
            let categories = stmt
                .query_map([&email], |row| {
                    Ok(Category {
                        id: row.get(0)?,
                        user_email: row.get(1)?,
                        name: row.get(2)?,
                        color: row.get(3)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(categories)
        })
        .await?;

    Ok(Json(categories))
}

async fn create(
    State(state): State<DbState>,
    Extension(claims): Extension<UserClaims>,
    Json(input): Json<CreateCategory>,
) -> AppResult<impl IntoResponse> {
    let category = Category {
        id: Uuid::now_v7().to_string(),
        user_email: Some(claims.email),
        name: input.name,
        color: input.color,
    };

    let category_clone = category.clone();
    state
        .conn
        .call(move |conn| {
            conn.execute(
                "INSERT INTO categories (id, user_email, name, color) VALUES (?1, ?2, ?3, ?4)",
                (
                    &category_clone.id,
                    &category_clone.user_email,
                    &category_clone.name,
                    &category_clone.color,
                ),
            )?;
            Ok(())
        })
        .await?;

    Ok((StatusCode::CREATED, Json(category)))
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
            // Only allow deleting user's custom categories (user_email IS NOT NULL)
            let rows = conn.execute(
                "DELETE FROM categories WHERE id = ?1 AND user_email = ?2",
                [&id, &email],
            )?;
            Ok(rows > 0)
        })
        .await?;

    if !deleted {
        return Err(AppError::Validation(
            "Category not found or cannot be deleted".to_string(),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
