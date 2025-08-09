use std::sync::Arc;

use axum::{extract::State, Json};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{app_state::AppState, error::AppError, models::user::{NewUser, User}};

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, AppError> {
    payload.validate()?;

    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO users (id, email, name) VALUES (?, ?, ?)"
    )
    .bind(id.to_string())
    .bind(&payload.email)
    .bind(&payload.name)
    .execute(&state.mysql_pool)
    .await?;

    // Fetch to return consistent data (optional for MySQL without RETURNING)
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, name FROM users WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_one(&state.mysql_pool)
    .await?;

    Ok(Json(user))
}

pub async fn list_users(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, AppError> {
    let users: Vec<User> = sqlx::query_as::<_, User>("SELECT id, email, name FROM users")
        .fetch_all(&state.mysql_pool)
        .await?;
    Ok(Json(json!({ "data": users })))
}


