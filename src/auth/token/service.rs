use anyhow::{Context, Result, anyhow};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use super::claims::{self, Claims};

pub struct JwtService {
    secret_key: Vec<u8>,
}

impl JwtService {
    pub fn new(secret_key: &str) -> Self {
        return Self {
            secret_key: secret_key.as_bytes().to_vec(),
        };
    }

    pub fn generate_access_token(&self, user_id: i32) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(15))
            .context("Invalid time")?
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
        };

        encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.secret_key),
        )
        .context("Failed to encode token")
    }

    pub fn generate_refresh_token(&self, user_id: i32) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(7))
            .context("Invalid time")?
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
        };

        encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.secret_key),
        )
        .context("Failed to encode token")
    }

    pub fn validate_token(&self, token: &str) -> Result<i32> {
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        let token_value = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret_key),
            &validation,
        )
        .context("Invalid Token")?;

        if token_value.claims.exp < Utc::now().timestamp() as usize {
            return Err(anyhow!("Invalid Token"));
        }

        return Ok(token_value.claims.sub);
    }
}
