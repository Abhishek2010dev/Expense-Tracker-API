use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use super::claims::Claims;

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
}
