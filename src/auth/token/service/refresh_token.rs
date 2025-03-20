use anyhow::{Context, Ok, Result, anyhow};
use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};

use crate::auth::token::{
    self, claims::Claims, hash::hash_token, repository::refresh_token::RefreshTokenRepository,
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

    pub async fn generate_token(&self, user_id: i32) -> Result<String> {
        let duration = Duration::days(7);
        let claims = Claims {
            sub: user_id,
            exp: Self::generate_expiration(duration)?,
        };

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

    pub async fn validate_token(&self, token: &str) -> Result<Claims> {
        let cliams = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret_key),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(|err| match err.kind() {
            ErrorKind::ExpiredSignature => anyhow!("Token has expired"),
            ErrorKind::InvalidToken => anyhow!("Invalid token format"),
            ErrorKind::InvalidSignature => anyhow!("Invalid token signature"),
            _ => anyhow!("Token validation failed"),
        })?;

        let redis_token = self
            .repository
            .get_refresh_token(cliams.sub)
            .await?
            .map(|v| hash_token(&v))
            .context("Redis token is null")?;

        if matches!(redis_token, token) {}

        todo!()
    }

    fn generate_expiration(duration: Duration) -> anyhow::Result<usize> {
        Utc::now()
            .checked_add_signed(duration)
            .map(|it| it.timestamp() as usize)
            .context("Invalid time")
    }
}
