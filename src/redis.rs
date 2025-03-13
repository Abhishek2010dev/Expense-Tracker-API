use anyhow::Result;
use fred::prelude::Client;

pub trait CacheConnection {
    async fn connect(url: &str) -> Result<Self>
    where
        Self: Sized;

    fn client(&self) -> Client;
}
