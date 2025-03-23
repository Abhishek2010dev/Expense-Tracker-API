use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::models::ExpenseCategory;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateExpensePayload {
    #[validate(custom(function = "validate_category"))]
    pub category: ExpenseCategory,

    #[validate(custom(function = "validate_amount"))]
    pub amount: BigDecimal,

    #[validate(length(max = 255, message = "Description too long"))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateExpensePayload {
    #[validate(length(min = 1))]
    pub id: i32,

    #[validate(custom(function = "validate_category"))]
    pub category: ExpenseCategory,

    #[validate(custom(function = "validate_amount"))]
    pub amount: BigDecimal,

    #[validate(length(max = 255, message = "Description too long"))]
    pub description: Option<String>,
}

fn validate_amount(amount: &BigDecimal) -> Result<(), ValidationError> {
    let min = BigDecimal::from(0);
    if amount <= &min {
        return Err(ValidationError::new("amount_must_be_greater_than_zero"));
    }
    Ok(())
}

fn validate_category(category: &ExpenseCategory) -> Result<(), ValidationError> {
    if !matches!(
        category,
        ExpenseCategory::Groceries
            | ExpenseCategory::Leisure
            | ExpenseCategory::Electronics
            | ExpenseCategory::Utilities
            | ExpenseCategory::Clothing
            | ExpenseCategory::Health
            | ExpenseCategory::Others
    ) {
        return Err(ValidationError::new("invalid_category"));
    }
    Ok(())
}
