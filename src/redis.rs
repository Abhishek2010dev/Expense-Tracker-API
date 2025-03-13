use std::time::Duration;

use anyhow::{Context, Ok, Result};
use fred::prelude::{Builder, Client, ClientLike, Config, EventInterface, TcpConfig};

pub trait CacheConnection {
    async fn connect(url: &str) -> Result<Self>
    where
        Self: Sized;

    fn client(&self) -> &Client;
}

pub struct RedisClient(Client);

impl CacheConnection for RedisClient {
    async fn connect(url: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let config = Config::from_url(url).context("Failed to create Redis Config")?;
        let client = Builder::from_config(config)
            .with_connection_config(|config| {
                config.connection_timeout = Duration::from_secs(5);
                config.tcp = TcpConfig {
                    nodelay: Some(true),
                    ..Default::default()
                }
            })
            .build()
            .context("Failed to build Redis client")?;
        client
            .init()
            .await
            .context("Failed to run `init` function for redis")?;

        tracing::debug!("Connected to redis");
        Ok(Self(client))
    }

    fn client(&self) -> &Client {
        &self.0
    }
}
