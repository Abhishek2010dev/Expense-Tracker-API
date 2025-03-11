use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    database_url: String,
    access_secret: String,
    refresh_secret: String,
    redis_url: String,
    port: u16,
}

impl AppConfig {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL is missing".to_string())?,
            access_secret: env::var("ACCESS_SECRET")
                .map_err(|_| "ACCESS_SECRET is missing".to_string())?,
            refresh_secret: env::var("REFRESH_SECRET")
                .map_err(|_| "REFRESH_SECRET is missing".to_string())?,
            redis_url: env::var("REDIS_URL").map_err(|_| "REDIS_URL is missing".to_string())?,
            port: env::var("PORT")
                .map_err(|_| "PORT is missing".to_string())?
                .parse::<u16>()
                .map_err(|_| "PORT must be a valid u16 number".to_string())?,
        })
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn access_secret(&self) -> &str {
        &self.access_secret
    }

    pub fn refresh_secret(&self) -> &str {
        &self.refresh_secret
    }

    pub fn redis_url(&self) -> &str {
        &self.redis_url
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
