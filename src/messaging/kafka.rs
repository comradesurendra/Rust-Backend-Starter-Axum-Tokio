use crate::{config::Settings, error::AppError};
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;

pub fn init_kafka_producer(settings: &Settings) -> Result<FutureProducer, AppError> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &settings.kafka.brokers)
        .set("message.timeout.ms", "5000")
        .create()?;
    Ok(producer)
}


