use mongodb::error::Error;
use repositories::{get_mongo_client, transaction::TransactionRepository};

pub mod extractors;
pub mod model;
pub mod repositories;

#[derive(Clone)]
pub struct AppState {
    pub transactions: TransactionRepository,
}

pub async fn init_state() -> Result<AppState, Error> {
    let database = get_mongo_client().await?.database("pocket_planner");

    let state = AppState {
        transactions: TransactionRepository::new(&database),
    };

    Ok(state)
}
