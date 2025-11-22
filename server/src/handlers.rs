use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use share::RegisterUser;
use sqlx::PgPool;

use crate::auth::*;

pub async fn register(
    State(pool): State<PgPool>,
    Json(user_data): Json<RegisterUser>,
) -> Result<impl IntoResponse, String> {
    println!("{user_data:?}");
    let existing = sqlx::query!("SELECT id FROM users WHERE email = $1", user_data.email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;
    println!("existing: {existing:?}");

    if existing.is_some() {
        return Ok((StatusCode::CONFLICT, Json("Увы".to_string())));
    }

    let password_hash = hash_password(&user_data.password).map_err(|e| e.to_string())?;

    println!("existing: {existing:?}");
    let user = sqlx::query!(
        "INSERT INTO users (username, email, bio, password_hash) VALUES ($1, $2, $3, $4);",
        user_data.username,
        user_data.email,
        " ",
        password_hash
    )
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;
    println!("user: {user:?}");

    Ok((StatusCode::OK, Json(user_data.email)))
}
