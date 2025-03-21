use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    auth::token::service::{access_token::AccessTokenService, refresh_token::RefreshTokenService},
    error::AppError,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct RefreshTokenPayload {
    refresh_token: String,
}

pub async fn refresh_token_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenPayload>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let user_id = state
        .refresh_token_service
        .validate_token(&payload.refresh_token)
        .await?
        .sub;

    let access_token = state.access_token_service.generate_token(user_id).await?;
    let response = serde_json::json!({
        "access_token": access_token,
    });

    Ok((StatusCode::OK, Json(response)))
}
