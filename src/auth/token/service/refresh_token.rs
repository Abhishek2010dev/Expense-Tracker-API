use anyhow::{Context, Ok, Result};
use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};

use crate::auth::token::{claims::Claims, repository::refresh_token::RefreshTokenRepository};

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

    pub async fn generate_access_token(&self, user_id: i32) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(15))
            .context("Invalid time")?
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
        };

        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.secret_key),
        )
        .context("Failed to encode refresh token")?;

        self.repository
            .store_refresh_token(user_id, &token, Duration::days(7).num_seconds())
            .await?;

        Ok(token)
    }

    fn generate_expiration() -> anyhow::Result<usize> {
        Utc::now()
            .checked_add_signed(Duration::minutes(15))
            .map(|it| it.timestamp() as usize)
            .context("Invalid time")
    }
}
