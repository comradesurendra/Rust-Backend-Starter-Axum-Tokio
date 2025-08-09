use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use secrecy::ExposeSecret;

mod api;
mod app_state;
mod config;
mod database;
mod error;
mod messaging;
mod models;
mod services;
mod telemetry;

use app_state::AppState;
use config::Settings;
use database::{mongo_db::init_mongo_client, my_sql::init_mysql_pool};
use messaging::{kafka::init_kafka_producer, rabbit_mq::init_rabbitmq};

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    dotenvy::dotenv().ok();

    telemetry::init_tracing();
    info!("starting service");

    let settings = Settings::load()?;

    // Initialize connectors
    let mysql_pool = init_mysql_pool(&settings).await?;
    let mongo_client = init_mongo_client(&settings).await?;
    let redis_client = redis::Client::open(settings.redis.uri.expose_secret().to_string())?;
    let (rabbit_conn, _rabbit_channel) = init_rabbitmq(&settings).await?;
    let kafka_producer = init_kafka_producer(&settings)?;

    let state = Arc::new(AppState {
        mysql_pool,
        mongo_client,
        redis_client,
        rabbit_conn,
        kafka_producer,
    });

    let app = build_router(state.clone());

    let addr: SocketAddr = format!(
        "{}:{}",
        settings.server.host, settings.server.port
    )
    .parse()
    .map_err(|e| error::AppError::Configuration(format!("invalid socket address: {}", e)))?;

    let listener = TcpListener::bind(addr).await?;
    info!(%addr, "listening");

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("shutdown complete");
    Ok(())
}

fn build_router(state: Arc<AppState>) -> Router {
    api::router(state)
}

async fn shutdown_signal() {
    // Wait for either SIGINT or SIGTERM
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = signal(SignalKind::terminate()).expect("install SIGTERM handler");
        let ctrl_c = tokio::signal::ctrl_c();
        tokio::select! {
            _ = ctrl_c => {},
            _ = sigterm.recv() => {},
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }

    // Small delay to give subsystems a chance to flush
    tokio::time::sleep(Duration::from_millis(200)).await;
}


