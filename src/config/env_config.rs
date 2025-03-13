use super::{env_provider::EnvProvider, error::ConfigError};
use std::borrow::Cow;

pub trait Config {
    fn database_url(&self) -> &str;
    fn access_secret(&self) -> &str;
    fn refresh_secret(&self) -> &str;
    fn redis_url(&self) -> &str;
    fn host(&self) -> &str;
    fn port(&self) -> u16;
}

#[derive(Debug, Clone)]
pub struct EnvConfig {
    database_url: Cow<'static, str>,
    host: Cow<'static, str>,
    access_secret: Cow<'static, str>,
    refresh_secret: Cow<'static, str>,
    redis_url: Cow<'static, str>,
    port: u16,
}

impl EnvConfig {
    pub fn new<P: EnvProvider>(provider: P) -> Result<Self, ConfigError> {
        Ok(Self {
            database_url: provider.get("DATABASE_URL")?,
            host: provider.get("HOST")?,
            access_secret: provider.get("ACCESS_SECRET")?,
            refresh_secret: provider.get("REFRESH_SECRET")?,
            redis_url: provider.get("REDIS_URL")?,
            port: provider
                .get("PORT")?
                .parse::<u16>()
                .map_err(ConfigError::InvalidPort)?,
        })
    }
}

impl Config for EnvConfig {
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

    fn host(&self) -> &str {
        &self.host
    }

    fn port(&self) -> u16 {
        self.port
    }
}
