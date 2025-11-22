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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tags {
    pub id: i32,
    pub name: String,
    pub custom: bool
}
