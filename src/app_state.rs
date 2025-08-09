use mongodb::Client as MongoClient;
use rdkafka::producer::FutureProducer;
use sqlx::MySqlPool;

pub struct AppState {
    pub mysql_pool: MySqlPool,
    pub mongo_client: MongoClient,
    pub redis_client: redis::Client,
    pub rabbit_conn: lapin::Connection,
    pub kafka_producer: FutureProducer,
}


