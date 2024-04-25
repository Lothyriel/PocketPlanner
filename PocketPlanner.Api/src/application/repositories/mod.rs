use mongodb::{options::ClientOptions, Client};

pub mod transaction;

pub async fn get_mongo_client() -> mongodb::error::Result<Client> {
    dotenv::dotenv().ok();

    let connection_string = std::env::var("MONGO_CONNECTION_STRING").unwrap_or_else(|_| {
        "mongodb://pocket-planner:P@SSW0RD@localhost/?retryWrites=true".to_string()
    });

    let options = ClientOptions::parse(connection_string).await?;

    Client::with_options(options)
}

type DbResult<T> = Result<T, DatabaseError>;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("IO Error: {0}")]
    IO(#[from] mongodb::error::Error),
    #[error("Expected id to be ObjectId")]
    ExpectedObjectId,
}
