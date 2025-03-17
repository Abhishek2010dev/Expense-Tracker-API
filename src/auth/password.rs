use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub trait PasswordService {
    fn hash_password(&self, password: &str) -> anyhow::Result<String>;
    fn verify_password(&self, password: &str, hash: &str) -> bool;
}

#[derive(Default)]
pub struct PasswordServiceImpl<'a> {
    argon2: Arc<Argon2<'a>>,
}

impl<'a> PasswordService for PasswordServiceImpl<'a> {
    fn hash_password(&self, password: &str) -> Result<String> {
        self.argon2
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .map(|value| value.to_string())
            .map_err(|err| anyhow!(err))
            .context("Failed to hash password")
    }

    fn verify_password(&self, password: &str, hash: &str) -> bool {
        PasswordHash::new(hash)
            .and_then(|parsed_hash| {
                self.argon2
                    .verify_password(password.as_bytes(), &parsed_hash)
            })
            .is_ok()
    }
}
