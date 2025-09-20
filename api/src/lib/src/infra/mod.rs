pub mod transaction;

#[derive(Clone)]
pub struct DbState {}

impl DbState {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct UserClaims {
    pub email: String,
    pub name: String,
    pub picture: String,
}
