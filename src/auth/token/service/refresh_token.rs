use anyhow::Context;
use std::sync::Arc;

use chrono::Duration;
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::auth::token::{
    claims::Claims,
    error::TokenValidationError,
    repository::refresh_token::RefreshTokenRepository,
    utils::{decode_token, hash_token},
};

pub struct RefreshTokenService<R: RefreshTokenRepository> {
    repository: R,
    secret_key: Arc<Vec<u8>>,
}

impl<R: RefreshTokenRepository> RefreshTokenService<R> {
    pub fn new(repository: R, secret_key: impl Into<Vec<u8>>) -> Self {
        return Self {
            repository,
            secret_key: Arc::new(secret_key.into()),
        };
    }

    pub async fn generate_token(&self, user_id: i32) -> anyhow::Result<String> {
        let duration = Duration::days(7);
        let claims = Claims::new(user_id, duration)?;

        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.secret_key),
        )
        .context("Failed to encode refresh token")?;

        self.repository
            .store_refresh_token(user_id, &token, duration.num_seconds())
            .await?;

        Ok(token)
    }

    pub async fn validate_token(&self, token: &str) -> Result<Claims, TokenValidationError> {
        let claims = decode_token(&self.secret_key, token)?;
        let redis_token = self
            .repository
            .get_refresh_token(claims.sub)
            .await
            .map_err(|_| TokenValidationError::ValidationFailed)?
            .ok_or(TokenValidationError::RedisTokenNull)?;

        if redis_token != hash_token(token) {
            return Err(TokenValidationError::ValidationFailed);
        }

        Ok(claims)
    }
}
