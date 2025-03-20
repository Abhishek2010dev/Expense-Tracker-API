use anyhow::Ok;
use chrono::Duration;
use serde::{Deserialize, Serialize};

use super::utils::generate_expiration;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: i32, duration: Duration) -> anyhow::Result<Self> {
        Ok(Self {
            sub,
            exp: generate_expiration(duration)?,
        })
    }
}
