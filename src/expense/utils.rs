use serde::{Deserialize, Serialize};

use super::models::ExpenseCategory;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExpensePayload {
    pub user_id: i32,
    pub category: ExpenseCategory,
    pub amount: f64,
    pub description: Option<String>,
}
