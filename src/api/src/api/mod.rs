mod calculations;
mod user;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/calculations", calculations::router())
        .nest("/user", user::router())
}
