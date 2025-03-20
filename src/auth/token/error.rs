use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenValidationError {
    #[error("Token has expired")]
    Expired,

    #[error("Invalid token format")]
    InvalidFormat,

    #[error("Invalid token signature")]
    InvalidSignature,

    #[error("Token validation failed")]
    ValidationFailed,

    #[error("Redis token is null")]
    RedisTokenNull,
}
