use super::{model::User, utils::CreateUserPayload};
use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[async_trait]
pub trait UserRespository {
    async fn create(&self, payload: CreateUserPayload) -> Result<User>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>>;
    async fn exits(&self, id: i32) -> Result<bool>;
}

pub struct UserRespositoryImpl {
    pool: Pool<Postgres>,
}

#[async_trait]
impl UserRespository for UserRespositoryImpl {
    async fn create(&self, payload: CreateUserPayload) -> Result<User> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING *;",
            payload.name,
            payload.email,
            payload.password_hash
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to create user")
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .context(format!("Failed to find user by id: {}", id))?;
        Ok(user)
    }

    async fn exits(&self, id: i32) -> Result<bool> {
        let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(exists.unwrap_or(false))
    }
}
