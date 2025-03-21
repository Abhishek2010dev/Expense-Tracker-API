use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use validator::Validate;

use crate::{
    auth::{
        password::PasswordService,
        token::{
            response::RefreshTokenResponse,
            service::{access_token::AccessTokenService, refresh_token::RefreshTokenService},
        },
    },
    error::AppError,
    state::AppState,
    user::repository::UserRepository,
    validation::ValidatedJson,
};

#[derive(Debug, Validate, Deserialize)]
pub struct LoginPayload {
    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: String,
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<LoginPayload>,
) -> Result<(StatusCode, Json<RefreshTokenResponse>), AppError> {
    let user = state
        .user_repository
        .find_by_email(&payload.email)
        .await?
        .ok_or(AppError::BadRequest("User does not exit".into()))?;

    if !state
        .password_service
        .verify_password(&payload.password, &user.password_hash)
    {
        return Err(AppError::Unauthorized("Invalid credentials".into()));
    }

    let response = RefreshTokenResponse {
        access_token: state.access_token_service.generate_token(user.id).await?,
        refresh_token: state.refresh_token_service.generate_token(user.id).await?,
    };
    Ok((StatusCode::OK, Json(response)))
}
