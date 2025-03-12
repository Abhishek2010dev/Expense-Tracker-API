use std::{borrow::Cow, env};

use super::{Config, error::ConfigError};

#[derive(Debug, Clone)]
pub struct EnvConfig {
    database_url: Cow<'static, str>,
    access_secret: Cow<'static, str>,
    refresh_secret: Cow<'static, str>,
    redis_url: Cow<'static, str>,
    port: u16,
}

impl EnvConfig {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self {
            database_url: get_env("DATABASE_URL")?,
            access_secret: get_env("ACCESS_SECRET")?,
            refresh_secret: get_env("REFRESH_SECRET")?,
            redis_url: get_env("REDIS_URL")?,
            port: get_env("PORT")?
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

    fn port(&self) -> u16 {
        self.port
    }
}

fn get_env(var: &'static str) -> Result<Cow<'static, str>, ConfigError> {
    env::var(var)
        .map(Cow::Owned)
        .map_err(|_| ConfigError::MissingVar(var))
}
