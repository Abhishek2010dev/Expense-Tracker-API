use jsonwebtoken::{DecodingKey, Validation, decode, errors::ErrorKind};
use sha2::{Digest, Sha256};

use super::{claims::Claims, error::TokenValidationError};

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.update(token);
    format!("{:x}", hasher.finalize())
}

pub fn decode_token(secret_key: &[u8], token: &str) -> Result<Claims, TokenValidationError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|err| match err.kind() {
        ErrorKind::ExpiredSignature => TokenValidationError::Expired,
        ErrorKind::InvalidToken => TokenValidationError::InvalidFormat,
        ErrorKind::InvalidSignature => TokenValidationError::InvalidSignature,
        _ => TokenValidationError::ValidationFailed,
    })
}
