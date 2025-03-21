use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("{0}")]
    Conflict(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(#[from] anyhow::Error),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
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
                    AppError::NotFound(message) => (StatusCode::NOT_FOUND, message.as_str()),
                    AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message.as_str()),
                    AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.as_str()),
                    AppError::InternalServerError(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "An internal server error occurred.",
                    ),
                    AppError::ValidationError(_) => unreachable!(), // Already handled above
                };
                let body = Json(json!({ "error": error_message }));
                (status, body).into_response()
            }
        }
    }
}
