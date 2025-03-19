use anyhow::{Context, Result, anyhow};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};

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
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret_key),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        ) {
            Ok(data) => Ok(data.claims.sub),
            Err(err) if matches!(err.kind(), ErrorKind::ExpiredSignature) => {
                Err(anyhow!("Token has expired"))
            }
            Err(_) => Err(anyhow!("Invalid token")),
        }
    }
}
