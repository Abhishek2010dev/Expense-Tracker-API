use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

use crate::{
    auth::token::{
        response::RefreshTokenResponse,
        service::{access_token::AccessTokenService, refresh_token::RefreshTokenService},
    },
    error::AppError,
    state::AppState,
    validation::ValidatedJson,
};

#[derive(Debug, Deserialize)]
pub struct RefreshTokenPayload {
    refresh_token: String,
}

pub async fn refresh_token_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<RefreshTokenPayload>,
) -> Result<(StatusCode, Json<RefreshTokenResponse>), AppError> {
    let user_id = state
        .refresh_token_service
        .validate_token(&payload.refresh_token)
        .await?;

    let response = RefreshTokenResponse {
        access_token: state.access_token_service.generate_token(user_id).await?,
        refresh_token: state.refresh_token_service.generate_token(user_id).await?,
    };

    Ok((StatusCode::OK, Json(response)))
}
