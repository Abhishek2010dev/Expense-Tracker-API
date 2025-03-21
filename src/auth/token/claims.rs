use async_trait::async_trait;
use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use chrono::Duration;
use serde::{Deserialize, Serialize};

use crate::{
    auth::token::service::access_token::AccessTokenService, error::AppError, state::AppState,
};

use super::utils::generate_expiration;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: i32, duration: Duration) -> anyhow::Result<Self> {
        Ok(Self {
            sub,
            exp: generate_expiration(duration)?,
        })
    }
}

impl<S> FromRequestParts<S> for Claims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::BadRequest("Invalid token".into()))?;

        match state
            .access_token_service
            .validate_token(bearer.token())
            .await
        {
            Ok(claims) => Ok(claims),
            Err(err) => match err {
                crate::auth::token::error::TokenValidationError::Expired => {
                    Err(AppError::Unauthorized("Token has expired".into()))
                }
                crate::auth::token::error::TokenValidationError::InvalidFormat => {
                    Err(AppError::BadRequest("Token format is invalid".into()))
                }
                crate::auth::token::error::TokenValidationError::InvalidSignature => {
                    Err(AppError::Unauthorized("Token signature is invalid".into()))
                }
                crate::auth::token::error::TokenValidationError::ValidationFailed => {
                    Err(AppError::Unauthorized("Token validation failed".into()))
                }
                crate::auth::token::error::TokenValidationError::RedisTokenNull => unreachable!(),
            },
        }
    }
}
