use mongodb::{error::Result, options::ClientOptions, Client};

mod transaction;

pub async fn get_mongo_client() -> Result<Client> {
    dotenv::dotenv().ok();

    let connection_string = std::env::var("MONGO_CONNECTION_STRING").unwrap_or_else(|_| {
        "mongodb://pocket-planner:P@SSW0RD@localhost/?retryWrites=true".to_string()
    });

    let options = ClientOptions::parse(connection_string).await?;

    Client::with_options(options)
}
