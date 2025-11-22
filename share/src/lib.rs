use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser{
    pub email: String,
    pub password: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags {
    pub id: i32,
    pub name: String,
    pub custom: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: String,
}
