use std::sync::Arc;
use axum::{routing::get, Router};

use crate::app_state::AppState;

pub mod health_check;
pub mod v1;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check::health))
        .nest("/api/v1", v1::router())
        .with_state(state)
}


