use std::sync::Arc;
use axum::{routing::get, Router};
use crate::app_state::AppState;

pub mod user_handler;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(user_handler::list_users).post(user_handler::create_user))
}


