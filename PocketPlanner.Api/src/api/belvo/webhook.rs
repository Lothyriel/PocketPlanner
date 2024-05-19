use axum::Json;

pub async fn handler() -> Json<()> {
    Json(())
}
