use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use share::{LoginUser, RegisterUser};
use sqlx::PgPool;

use crate::auth::hash_password;

pub async fn register(
    State(pool): State<PgPool>,
    Json(user_data): Json<RegisterUser>,
) -> Result<impl IntoResponse, String> {
    let existing = sqlx::query!("SELECT id FROM users WHERE email = $1", user_data.email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if existing.is_some() {
        return Ok((StatusCode::CONFLICT, Json("Увы".to_string())));
    }

    let password_hash = hash_password(&user_data.password);

    let _user = sqlx::query!(
        "INSERT INTO users (username, email, bio, password_hash) VALUES ($1, $2, $3, $4);",
        user_data.username,
        user_data.email,
        " ",
        password_hash
    )
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok((StatusCode::OK, Json(user_data.email)))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(user_data): Json<LoginUser>,
) -> Result<impl IntoResponse, String> {
    let existing = sqlx::query!(
        "SELECT id FROM users WHERE email = $1 AND password_hash = $2;",
        user_data.email,
        hash_password(&user_data.password)
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| e.to_string())?;

    if existing.is_some() {
        return Ok((StatusCode::OK, Json(user_data.email)));
    }

    Ok((StatusCode::NOT_FOUND, Json("Увы".to_string())))
}
