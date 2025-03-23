use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    auth::token::claims::Claims, error::AppError, state::AppState, validation::ValidatedJson,
};

use super::{models::Expense, utils::CreateExpensePayload};

pub async fn create_expense_handler(
    claims: Claims,
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<CreateExpensePayload>,
) -> Result<(StatusCode, Json<Expense>), AppError> {
    let expense = state
        .expense_repository
        .create_expense(payload, claims.sub)
        .await?;
    Ok((StatusCode::CREATED, Json(expense)))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/expenses", post(create_expense_handler))
}
