use crate::{config::Settings, error::AppError};
use lapin::{options::*, types::FieldTable, Channel, Connection, ConnectionProperties};

pub async fn init_rabbitmq(settings: &Settings) -> Result<(Connection, Channel), AppError> {
    let conn = Connection::connect(settings.rabbitmq_uri(), ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    // Declare a default exchange/queue example (idempotent)
    let _ = channel
        .queue_declare(
            "example",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    Ok((conn, channel))
}


