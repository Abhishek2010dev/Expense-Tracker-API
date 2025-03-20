use anyhow::{Context, Result};
use async_trait::async_trait;
use fred::{
    prelude::{Client, KeysInterface},
    types::Expiration,
};
use std::sync::Arc;

#[async_trait]
pub trait RefreshTokenRepository {
    // ttl expiration time for token in second
    async fn store_refresh_token(&self, user_id: i32, token: &str, ttl: i64) -> Result<()>;
    async fn get_refresh_token(&self, user_id: i32) -> Result<Option<String>>;
    async fn delete_refresh_token(&self, user_id: i32) -> Result<()>;
}

pub struct RedisRefreshTokenRepository {
    client: Arc<Client>,
}

impl RedisRefreshTokenRepository {
    pub fn new(client: Arc<Client>) -> RedisRefreshTokenRepository {
        return RedisRefreshTokenRepository { client };
    }
}

#[async_trait]
impl RefreshTokenRepository for RedisRefreshTokenRepository {
    async fn store_refresh_token(&self, user_id: i32, token: &str, ttl: i64) -> Result<()> {
        self.client
            .set(
                format!("refresh_token:{}", user_id),
                token,
                Some(Expiration::EX(ttl)),
                None,
                false,
            )
            .await
            .context(format!("Failed to store refresh token with id: {user_id}"))
    }

    async fn get_refresh_token(&self, user_id: i32) -> Result<Option<String>> {
        self.client
            .get(format!("refresh_token:{}", user_id))
            .await
            .context(format!("Failed to get refresh token with id: {}", user_id))
    }

    async fn delete_refresh_token(&self, user_id: i32) -> Result<()> {
        self.client
            .del(format!("refresh_token:{}", user_id))
            .await
            .context(format!(
                "Failed to delete refresh token with id: {}",
                user_id
            ))
    }
}
