use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

use crate::auth::token::error::TokenValidationError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(#[from] anyhow::Error),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("{}", self);

        match &self {
            AppError::ValidationError(errors) => {
                let body = Json(json!({
                    "error": "Validation failed",
                    "details": errors.field_errors()
                }));
                (StatusCode::BAD_REQUEST, body).into_response()
            }
            _ => {
                let (status, error_message) = match &self {
                    AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
                    AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
                    AppError::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
                    AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
                    AppError::InternalServerError(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &"An internal server error occurred.".to_string(),
                    ),
                    AppError::ValidationError(_) => unreachable!(),
                    AppError::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, &self.to_string()),
                };
                let body = Json(json!({ "error": error_message }));
                (status, body).into_response()
            }
        }
    }
}

impl From<TokenValidationError> for AppError {
    fn from(value: TokenValidationError) -> Self {
        match value {
            crate::auth::token::error::TokenValidationError::Expired => {
                AppError::Unauthorized("Token has expired".into())
            }
            crate::auth::token::error::TokenValidationError::InvalidFormat => {
                AppError::BadRequest("Token format is invalid".into())
            }
            crate::auth::token::error::TokenValidationError::InvalidSignature => {
                AppError::Unauthorized("Token signature is invalid".into())
            }
            crate::auth::token::error::TokenValidationError::ValidationFailed => {
                AppError::Unauthorized("Token validation failed".into())
            }
            crate::auth::token::error::TokenValidationError::RedisTokenNull => {
                AppError::Unauthorized("Token not found in Redis".into())
            }
        }
    }
}
