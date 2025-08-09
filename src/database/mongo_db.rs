use crate::{config::Settings, error::AppError};
use mongodb::{options::ClientOptions, Client};

pub async fn init_mongo_client(settings: &Settings) -> Result<Client, AppError> {
    let mut options = ClientOptions::parse(settings.mongodb_uri()).await?;
    // Optional: set an app name for observability
    options.app_name = Some("rust-backend".to_string());
    let client = Client::with_options(options)?;
    Ok(client)
}


