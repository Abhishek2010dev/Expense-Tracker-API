use std::sync::Arc;

use anyhow::Context;
use sqlx::PgPool;

use super::{
    models::{Expense, ExpenseCategory},
    utils::{CreateExpensePayload, UpdateExpensePayload},
};

pub struct ExpenseRepository {
    pool: Arc<PgPool>,
}

impl ExpenseRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        ExpenseRepository { pool }
    }
    pub async fn create_expense(
        &self,
        payload: CreateExpensePayload,
        user_id: i32,
    ) -> anyhow::Result<Expense> {
        sqlx::query_as!(
            Expense,
            r#"
    INSERT INTO expenses (user_id, category, amount, description)
    VALUES ($1, $2, $3, $4)
    RETURNING id, category AS "category: _", amount, description, expense_date;
    "#,
            user_id,
            payload.category as ExpenseCategory,
            payload.amount,
            payload.description,
        )
        .fetch_one(&*self.pool)
        .await
        .context("Failed to create expense")
    }

    pub async fn find_expenses(&self, user_id: i32) -> anyhow::Result<Vec<Expense>> {
        sqlx::query_as!(
            Expense,
            r#"
            SELECT id, category AS "category: _", amount, description, expense_date 
            FROM expenses WHERE user_id = $1;
            "#,
            user_id
        )
        .fetch_all(&*self.pool)
        .await
        .context(format!("Failed to get expense by user_id: {}", user_id))
    }

    pub async fn delete_expense(&self, id: i32, user_id: i32) -> anyhow::Result<Option<i32>> {
        sqlx::query_scalar!(
            "DELETE FROM expenses WHERE id = $1 AND user_id = $2 RETURNING id;",
            id,
            user_id
        )
        .fetch_optional(&*self.pool)
        .await
        .context(format!("Failed to delete expense by id: {}", id))
    }

    pub async fn find_expenses_by_category(
        &self,
        user_id: i32,
        category: ExpenseCategory,
    ) -> anyhow::Result<Vec<Expense>> {
        sqlx::query_as!(
            Expense,
            r#"
            SELECT id, category AS "category: _", amount, description, expense_date 
            FROM expenses WHERE user_id = $1 AND category = $2
            "#,
            user_id,
            category as ExpenseCategory
        )
        .fetch_all(&*self.pool)
        .await
        .context(format!(
            "Failed to get expenses with user_id {} and category {:?} ",
            user_id, category
        ))
    }

    pub async fn update_expense(
        &self,
        payload: UpdateExpensePayload,
    ) -> anyhow::Result<Option<Expense>> {
        sqlx::query_as!(
            Expense,
            r#"
    UPDATE expenses 
    SET category = COALESCE($1, category),
        amount = COALESCE($2, amount),
        description = COALESCE($3, description)
    WHERE id = $4
    RETURNING id, category AS "category: _", amount, description, expense_date;
    "#,
            payload.category as _,
            payload.amount,
            payload.description,
            payload.id
        )
        .fetch_optional(&*self.pool)
        .await
        .context("Failed to update expense")
    }
}
