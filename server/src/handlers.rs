use axum::{
    extract::State,
    Json
};
use sqlx::PgPool;

use crate::models::*;
use crate::auth::*;

pub async fn register(
    State(pool): State<PgPool>,
    Json(user_data): Json<CreateUser>,
    ) -> Result<Json<AuthResponse>, String> {
    let existing = sqlx::query!("SELECT id FROM users WHERE username = $1 AND email = $2",
        user_data.username, user_data.email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if existing.is_some() {
        return Err("User already exists.".to_string());
    }

    let password_hash = hash_password(&user_data.password)
        .map_err(|e| e.to_string())?;

    let user = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id, username,
    email, password_hash",
        user_data.username,
        user_data.email,
        password_hash
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

   let user_response = User {
        id: user.id,
        username: user.username.expect("Username should not be null"), 
        email: user.email.expect("Email should not be null"),
        bio: "".to_string(),
        password_hash: user.password_hash.expect("Password hash should not be null"),
    };

    Ok(Json(AuthResponse {
        user: user_response,
        message: "User created successfully".to_string(),
    }))
}
