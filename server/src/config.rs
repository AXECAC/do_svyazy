use sqlx::postgres::PgPoolOptions;
use std::env;

pub struct Config {
    pub database_url: String,
    pub secret_key: String,
    pub secret_refresh_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".into()),
            secret_key: env::var("JWT_SECRET").unwrap_or("key".into()),
            secret_refresh_key: env::var("JWT_SECRET_REFRESH").unwrap_or("key".into()),
        }
    }
}

pub async fn get_db_pool(database_url: &str) -> sqlx::PgPool {
    PgPoolOptions::new()
        // .max_connections(5) // Set maximum number of connections
        .connect(database_url)
        .await
        .unwrap()
}
