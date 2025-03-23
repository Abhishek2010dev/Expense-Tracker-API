use std::sync::Arc;

use anyhow::Context;
use sqlx::PgPool;

use super::{
    models::{Expense, ExpenseCategory},
    utils::CreateExpensePayload,
};

pub struct ExpenseRepository {
    pool: Arc<PgPool>,
}

impl ExpenseRepository {
    pub async fn create_expense(&self, payload: CreateExpensePayload) -> anyhow::Result<Expense> {
        sqlx::query_as!(
            Expense,
            r#"
    INSERT INTO expenses (user_id, category, amount, description)
    VALUES ($1, $2, $3, $4)
    RETURNING id, user_id, category AS "category: _", amount, description, expense_date
    "#,
            payload.user_id,
            payload.category as ExpenseCategory,
            payload.amount,
            payload.description,
        )
        .fetch_one(&*self.pool)
        .await
        .context("Failed to create expense")
    }
}
