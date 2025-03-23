use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::post,
};

use crate::{
    auth::token::claims::Claims, error::AppError, state::AppState, validation::ValidatedJson,
};

use super::{
    models::{Expense, ExpenseCategory},
    utils::CreateExpensePayload,
};

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

#[derive(serde::Deserialize)]
pub struct ExpenseCategoryQuery {
    pub category: Option<ExpenseCategory>,
}

pub async fn get_expenses(
    claims: Claims,
    State(state): State<Arc<AppState>>,
    Query(query): Query<ExpenseCategoryQuery>,
) -> Result<(StatusCode, Json<Vec<Expense>>), AppError> {
    let expenses = if let Some(category) = query.category {
        state
            .expense_repository
            .find_expenses_by_category(claims.sub, category)
            .await?
    } else {
        state.expense_repository.find_expenses(claims.sub).await?
    };

    Ok((StatusCode::OK, Json(expenses)))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/expenses", post(create_expense_handler).get(get_expenses))
}
