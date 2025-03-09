mod calculations;
mod user;

use axum::Router;

use crate::application::AppState;
pub use user::get_google_jwks;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/calculations", calculations::router())
        .nest("/user", user::router(state))
}
