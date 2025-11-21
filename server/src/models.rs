use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: String,
    #[serde(skip_serializing)]
    pub password_hash: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser{
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser{
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse{
    pub user: User,
    pub message: String,
}
