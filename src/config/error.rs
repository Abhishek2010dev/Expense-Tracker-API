use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingVar(&'static str),

    #[error("Invalid PORT: must be a valid u16 number")]
    InvalidPort(#[from] std::num::ParseIntError),
}
