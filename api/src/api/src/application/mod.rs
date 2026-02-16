use std::sync::Arc;

use jsonwebtoken::jwk::JwkSet;
use tokio::sync::RwLock;

pub mod extractors;
pub mod model;

#[derive(Clone)]
pub struct ApiState {
    pub google_keys: Arc<RwLock<JwkSet>>,
    pub audience: String,
    pub secure_env: bool,
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub access_ttl: u64,
    pub refresh_ttl: u64,
}
