use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod application;
mod extensions;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("debug,hyper=off".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));

    lib::init_db().expect("Init DB tables");

    let router = api::router().layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("To bind to {addr:?}");

    let server = axum::serve(listener, router);

    tracing::info!("Starting API in: {}", addr);

    if let Err(err) = server.await {
        tracing::error!("{}", err);
    }
}

type ResponseResult<T> = Result<Json<T>, ResponseError>;

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("HttpError: {0}")]
    Http(#[from] reqwest::Error),
    #[error("EnvError: {0}")]
    Environment(#[from] std::env::VarError),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            ResponseError::Http(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::Environment(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (code, Json(json!({"error": self.to_string() }))).into_response()
    }
}
