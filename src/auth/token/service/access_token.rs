use anyhow::Context;
use std::sync::Arc;

use chrono::Duration;
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::auth::token::{claims::Claims, error::TokenValidationError, utils::decode_token};

pub struct AccessTokenServiceImpl {
    secret_key: Arc<Vec<u8>>,
}

pub trait AccessTokenService {
    fn generate_token(&self, user_id: i32) -> anyhow::Result<String>;
    fn validate_token(&self, token: &str) -> Result<Claims, TokenValidationError>;
}

impl AccessTokenServiceImpl {
    pub fn new(secret_key: impl Into<Vec<u8>>) -> Self {
        Self {
            secret_key: Arc::new(secret_key.into()),
        }
    }
}

#[async_trait::async_trait]
impl AccessTokenService for AccessTokenServiceImpl {
    async fn generate_token(&self, user_id: i32) -> anyhow::Result<String> {
        let duration = Duration::hours(1); // Access tokens usually have a shorter lifespan
        let claims = Claims::new(user_id, duration)?;
        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.secret_key),
        )
        .context("Failed to encode access token")?;

        Ok(token)
    }

    async fn validate_token(&self, token: &str) -> Result<Claims, TokenValidationError> {
        decode_token(&self.secret_key, token)
    }
}
