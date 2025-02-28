use askama::Template;

mod views;

pub fn get_template(route: &str) -> Result<String, askama::Error> {
    match route {
        "/load-more" => views::content().render(),
        _ => views::error().render(),
    }
}
#[cfg(feature = "axum")]
pub mod axum {
    use super::views;
    use askama_axum::IntoResponse;
    use axum::{routing, Router};

    pub fn router() -> Router {
        Router::new().route("/load-more", routing::post(content))
    }

    async fn content() -> impl axum::response::IntoResponse {
        views::content().into_response()
    }
}
