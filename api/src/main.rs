use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

use application::repositories::DatabaseError;

mod api;
mod application;
mod extensions;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));

    let state = application::init_state()
        .await
        .expect("To initialize application state");

    let router = api::router(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("To bind to {addr:?}");

    let server = axum::serve(listener, router);

    log::info!("Starting API in: {}", addr);

    if let Err(err) = server.await {
        log::error!("{}", err);
    }
}

type ResponseResult<T> = Result<Json<T>, ResponseError>;

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("DatabaseError: {0}")]
    Database(#[from] DatabaseError),
    #[error("HttpError: {0}")]
    Http(#[from] reqwest::Error),
    #[error("EnvError: {0}")]
    Environment(#[from] std::env::VarError),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            ResponseError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::Http(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::Environment(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (code, Json(json!({"error": self.to_string() }))).into_response()
    }
}
