use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use server::model::User;
use share::Tokens;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,
    pub name: String,
    pub role: String,
    pub exp: usize,
    pub jti: Option<String>,
}

#[derive(Clone)]
pub struct CurrentUser(pub Claims);

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("SystemTime error: {0}")]
    DataTime(#[from] SystemTimeError),
    #[error("Token expired")]
    Expired,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Refresh token not found in database")]
    RefreshNotFound,
}

pub struct TokenSrvice {
    secret: String,
    secret_refresh: String,
    access_duration: usize,
    refresh_duration: usize,
    db: PgPool,
    issuer: String,
    audience: String,
}

impl TokenSrvice {
    /// Creates a new [`TokenSrvice`].
    pub fn new(secret: String, secret_refresh: String, pool: PgPool) -> Self {
        Self {
            secret,
            secret_refresh,
            db: pool,
            access_duration: 5,
            refresh_duration: 1440,
            issuer: "axecac-kek.ru".to_string(),
            audience: "axecac-kek.ru".to_string(),
        }
    }

    fn generate_access_token(&self, user: &User) -> Result<String, TokenError> {
        let exp = (SystemTime::now() + Duration::from_mins(self.access_duration as u64))
            .duration_since(UNIX_EPOCH)?
            .as_secs() as usize;

        let claims = Claims {
            sub: user.id,
            name: user.name.clone(),
            role: user.role.clone(),
            exp,
            jti: None,
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?)
    }

    fn generate_refresh_token(&self, user: &User) -> Result<String, TokenError> {
        let exp = (SystemTime::now() + Duration::from_mins(self.refresh_duration as u64))
            .duration_since(UNIX_EPOCH)?
            .as_secs() as usize;

        let claims = Claims {
            sub: user.id,
            name: user.name.clone(),
            role: user.role.clone(),
            exp,
            jti: Some(Uuid::new_v4().to_string()),
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_refresh.as_bytes()),
        )?)
    }

    pub async fn generate_tokens(&self, user: &User) -> Result<Tokens, TokenError> {
        let access_token = self.generate_access_token(user)?;
        let refresh_token = self.generate_refresh_token(user)?;

        let _ = sqlx::query!(
            "INSERT INTO refresh_tokens (user_id, token) VALUES ($1, $2) \
                ON CONFLICT (user_id) DO UPDATE SET token = $2",
            user.id,
            refresh_token
        )
        .execute(&self.db)
        .await?;

        Ok(Tokens {
            access_token,
            refresh_token,
        })
    }

    pub async fn refresh_tokens(&self) {}

    pub fn validate_access_token(&self, token: &str) -> Result<Claims, TokenError> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);

        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);
        validation.leeway = 0;
        validation.required_spec_claims.insert("exp".to_string());

        let token_data =
            decode::<Claims>(token, &decoding_key, &validation).map_err(TokenError::Jwt)?;

        Ok(token_data.claims)
    }

    fn validate_refresh_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(self.secret_refresh.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 0;

        let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}
