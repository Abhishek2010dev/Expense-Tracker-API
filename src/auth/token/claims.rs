use std::sync::Arc;

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
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = Arc::from_ref(state);

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::BadRequest("Invalid token".into()))?;

        state
            .access_token_service
            .validate_token(bearer.token())
            .await
            .map_err(|err| AppError::from(err))
    }
}
