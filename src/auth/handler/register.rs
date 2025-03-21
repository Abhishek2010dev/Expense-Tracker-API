use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use axum_valid::Valid;
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
    user::{repository::UserRepository, utils::CreateUserPayload},
};

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterPayload {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    name: String,

    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: String,
}

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Valid(Json(payload)): Valid<Json<RegisterPayload>>,
) -> Result<(StatusCode, Json<RefreshTokenResponse>), AppError> {
    if state
        .user_repository
        .exists_by_email(&payload.email)
        .await?
    {
        return Err(AppError::Conflict("User already exits".into()));
    }

    let password_hash = state.password_service.hash_password(&payload.password)?;
    let user = state
        .user_repository
        .create(CreateUserPayload {
            name: payload.name,
            email: payload.email,
            password_hash,
        })
        .await?;

    let response = RefreshTokenResponse {
        access_token: state.access_token_service.generate_token(user.id).await?,
        refresh_token: state.refresh_token_service.generate_token(user.id).await?,
    };
    Ok((StatusCode::CREATED, Json(response)))
}
