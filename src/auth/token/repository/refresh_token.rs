use anyhow::Result;
use async_trait::async_trait;
use fred::prelude::Client;
use std::sync::Arc;

#[async_trait]
pub trait RefreshTokenRepository {
    async fn store_refresh_token(&self, user_id: i32, token: &str, ttl: u64) -> Result<()>;
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
