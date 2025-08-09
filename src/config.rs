use crate::error::AppError;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub mysql: MySqlSettings,
    pub mongodb: MongoSettings,
    pub redis: RedisSettings,
    pub rabbitmq: RabbitSettings,
    pub kafka: KafkaSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MySqlSettings {
    pub uri: SecretString,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MongoSettings {
    pub uri: SecretString,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisSettings {
    pub uri: SecretString,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RabbitSettings {
    pub uri: SecretString,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaSettings {
    pub brokers: String,
}

impl Settings {
    pub fn load() -> Result<Self, AppError> {
        // Load .env first
        let _ = dotenvy::dotenv();

        let mut builder = ::config::Config::builder()
            .add_source(::config::File::with_name("config/default").required(false))
            .add_source(::config::Environment::with_prefix("APP").separator("__"));

        // Optional production overrides
        if std::env::var("APP_ENV").map(|v| v == "production").unwrap_or(false) {
            builder = builder.add_source(::config::File::with_name("config/production").required(false));
        }

        let cfg = builder.build().map_err(|e| AppError::Configuration(e.to_string()))?;
        let settings: Settings = cfg.try_deserialize().map_err(|e| AppError::Configuration(e.to_string()))?;
        Ok(settings)
    }

    pub fn mysql_uri(&self) -> &str {
        self.mysql.uri.expose_secret()
    }

    pub fn mongodb_uri(&self) -> &str {
        self.mongodb.uri.expose_secret()
    }

    pub fn redis_uri(&self) -> &str {
        self.redis.uri.expose_secret()
    }

    pub fn rabbitmq_uri(&self) -> &str {
        self.rabbitmq.uri.expose_secret()
    }
}


