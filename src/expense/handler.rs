use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, post},
};

use crate::{
    auth::token::claims::Claims, error::AppError, state::AppState, validation::ValidatedJson,
};

use super::{
    models::{Expense, ExpenseCategory},
    utils::{CreateExpensePayload, UpdateExpensePayload},
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

pub async fn delete_expense(
    claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    match state
        .expense_repository
        .delete_expense(id, claims.sub)
        .await?
    {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn update_expense(
    claims: Claims,
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<UpdateExpensePayload>,
) -> Result<(StatusCode, Json<Option<Expense>>), AppError> {
    match state
        .expense_repository
        .update_expense(claims.sub, payload)
        .await?
    {
        Some(v) => Ok((StatusCode::OK, Json(Some(v)))),
        None => Ok((StatusCode::NOT_FOUND, Json(None))),
    }
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/expenses",
            post(create_expense_handler)
                .get(get_expenses)
                .put(update_expense),
        )
        .route("/expenses/{id}", delete(delete_expense))
}
