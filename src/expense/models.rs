use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "expense_category", rename_all = "snake_case")]
pub enum ExpenseCategory {
    Groceries,
    Leisure,
    Electronics,
    Utilities,
    Clothing,
    Health,
    Others,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    id: i32,
    user_id: i32,
    category: ExpenseCategory,
    amount: f64,
    description: Option<String>,
    expense_date: NaiveDateTime,
}
