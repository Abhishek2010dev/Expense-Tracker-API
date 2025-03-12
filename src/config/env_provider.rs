use std::{borrow::Cow, env};

use super::error::ConfigError;

pub trait EnvProvider {
    fn get(&self, key: &'static str) -> Result<Cow<'static, str>, ConfigError>;
}

pub struct StdEnv;

impl EnvProvider for StdEnv {
    fn get(&self, key: &'static str) -> Result<Cow<'static, str>, ConfigError> {
        env::var(key)
            .map(Cow::Owned)
            .map_err(|_| ConfigError::MissingVar(key))
    }
}
