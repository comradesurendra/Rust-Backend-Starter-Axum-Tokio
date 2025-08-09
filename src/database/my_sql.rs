use crate::{config::Settings, error::AppError};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub async fn init_mysql_pool(settings: &Settings) -> Result<MySqlPool, AppError> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(settings.mysql_uri())
        .await?;
    Ok(pool)
}


