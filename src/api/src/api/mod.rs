mod calculations;
mod user;

use axum::Router;

use crate::application::ApiState;
pub use user::get_google_jwks;

pub fn router(state: ApiState) -> Router {
    Router::new()
        .nest("/calculations", calculations::router())
        .nest("/user", user::router(state))
}
