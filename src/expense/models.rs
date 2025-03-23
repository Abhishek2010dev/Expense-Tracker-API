use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "expense_category", rename_all = "PascalCase")]
pub enum ExpenseCategory {
    Groceries,
    Leisure,
    Electronics,
    Utilities,
    Clothing,
    Health,
    Others,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Expense {
    pub id: i32,
    pub category: ExpenseCategory,
    pub amount: BigDecimal,
    pub description: Option<String>,
    pub expense_date: NaiveDateTime,
}
