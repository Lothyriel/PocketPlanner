use std::sync::Arc;

use jsonwebtoken::jwk::JwkSet;
use tokio::sync::RwLock;

pub mod extractors;
pub mod model;

#[derive(Clone)]
pub struct ApiState {
    pub google_keys: Arc<RwLock<JwkSet>>,
    pub audiences: Vec<String>,
    pub secure_env: bool,
}
