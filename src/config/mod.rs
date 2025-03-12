pub mod env_config;
pub mod error;

pub trait Config {
    fn database_url(&self) -> &str;
    fn access_secret(&self) -> &str;
    fn refresh_secret(&self) -> &str;
    fn redis_url(&self) -> &str;
    fn port(&self) -> u16;
}
