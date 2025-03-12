pub mod env_config;
pub mod error;

pub trait Config {
    fn database_url(&self) -> &str;
    fn access_secret(&self) -> &str;
    fn refresh_secret(&self) -> &str;
    fn redis_url(&self) -> &str;
    fn port(&self) -> u16;
}

use env_config::EnvProvider;
use error::ConfigError;
use std::{borrow::Cow, env};

#[derive(Debug, Clone)]
pub struct EnvConfig<P: EnvProvider> {
    provider: P,
    database_url: Cow<'static, str>,
    access_secret: Cow<'static, str>,
    refresh_secret: Cow<'static, str>,
    redis_url: Cow<'static, str>,
    port: u16,
}

impl<P: EnvProvider> EnvConfig<P> {
    pub fn new(provider: P) -> Result<Self, ConfigError> {
        Ok(Self {
            database_url: provider.get("DATABASE_URL")?,
            access_secret: provider.get("ACCESS_SECRET")?,
            refresh_secret: provider.get("REFRESH_SECRET")?,
            redis_url: provider.get("REDIS_URL")?,
            port: provider
                .get("PORT")?
                .parse::<u16>()
                .map_err(ConfigError::InvalidPort)?,
            provider,
        })
    }
}

impl<P: EnvProvider> Config for EnvConfig<P> {
    fn database_url(&self) -> &str {
        &self.database_url
    }

    fn access_secret(&self) -> &str {
        &self.access_secret
    }

    fn refresh_secret(&self) -> &str {
        &self.refresh_secret
    }

    fn redis_url(&self) -> &str {
        &self.redis_url
    }

    fn port(&self) -> u16 {
        self.port
    }
}
