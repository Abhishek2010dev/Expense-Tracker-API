use std::sync::Arc;

use super::{model::User, utils::CreateUserPayload};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, payload: CreateUserPayload) -> Result<User>;
    async fn exists_by_email(&self, email: &str) -> Result<bool>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
}

pub struct UserRepositoryImpl {
    pool: Arc<Pool<Postgres>>,
}

impl UserRepositoryImpl {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, payload: CreateUserPayload) -> Result<User> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING *;",
            payload.name,
            payload.email,
            payload.password_hash
        )
        .fetch_one(&*self.pool)
        .await
        .context("Failed to create user")
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&*self.pool)
            .await
            .with_context(|| format!("Failed to find user by email: {}", email))
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool> {
        let exists =
            sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", email)
                .fetch_one(&*self.pool)
                .await
                .context("Failed to check if user exists")?;

        Ok(exists.unwrap_or(false))
    }
}
