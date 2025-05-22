use std::sync::Arc;

use anyhow::Result;
use application::ApiState;
use axum::{response::IntoResponse, Json, Router};
use lib::infra::{Db, DbState};
use reqwest::StatusCode;
use serde_json::json;
use surrealdb::{engine::any, opt::auth::Root};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
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

    dotenvy::dotenv().ok();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));

    let jwkset = api::get_google_jwks().await.expect("Get google JWKset");

    let db = get_db().await.expect("Get SurrealDb conn");

    let state = DbState::new(db);

    let api_state = ApiState {
        google_keys: Arc::new(RwLock::new(jwkset)),
    };

    let router = router(state, api_state).layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("To bind to {addr:?}");

    let server = axum::serve(listener, router);

    tracing::info!("Starting API in: {}", addr);

    if let Err(err) = server.await {
        tracing::error!("{}", err);
    }
}

async fn get_db() -> Result<Db> {
    let db_adrr = std::env::var("SURREAL_DB_ADDR")?;

    let db = any::connect(db_adrr).await?;

    db.use_ns("pp").await?;

    let password = &std::env::var("SURREAL_DB_PASS")?;
    let username = &std::env::var("SURREAL_DB_USER")?;

    db.signin(Root { username, password }).await?;

    Ok(db)
}

pub fn router(state: DbState, api_state: ApiState) -> Router {
    let auth_layer = axum::middleware::from_fn_with_state(api_state.clone(), api::auth);

    lib::router(state)
        .layer(auth_layer)
        .nest("/api", api::router(api_state))
        .fallback_service(ServeDir::new("public"))
}

type ResponseResult<T> = Result<Json<T>, ResponseError>;

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("HttpError: {0}")]
    Http(#[from] reqwest::Error),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            ResponseError::Http(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (code, Json(json!({"error": self.to_string() }))).into_response()
    }
}
