use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    Configuration(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("mysql error: {0}")]
    MySql(#[from] sqlx::Error),
    #[error("mongo error: {0}")]
    Mongo(#[from] mongodb::error::Error),
    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("rabbitmq error: {0}")]
    Rabbit(#[from] lapin::Error),
    #[error("kafka error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),
    #[error("validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    #[error("serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("unexpected error: {0}")]
    Unexpected(String),
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Configuration(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::MySql(_) => (StatusCode::INTERNAL_SERVER_ERROR, "database error".to_string()),
            AppError::Mongo(_) => (StatusCode::INTERNAL_SERVER_ERROR, "database error".to_string()),
            AppError::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, "cache error".to_string()),
            AppError::Rabbit(_) => (StatusCode::INTERNAL_SERVER_ERROR, "messaging error".to_string()),
            AppError::Kafka(_) => (StatusCode::INTERNAL_SERVER_ERROR, "messaging error".to_string()),
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::SerdeJson(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Unexpected(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, Json(ErrorBody { error: message })).into_response()
    }
}


