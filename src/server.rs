use anyhow::{Context, Ok};
use axum::Router;
use sqlx::Postgres;
use tokio::net::TcpListener;

use crate::{
    config::Config,
    database::{DatabaseConnection, PgDatabase},
    redis::{CacheConnection, RedisClient},
};

pub struct Server<C, D, R>
where
    C: Config,
    D: DatabaseConnection<Postgres>,
    R: CacheConnection,
{
    config: C,
    db: D,
    redis: R,
}
impl<C: Config> Server<C, PgDatabase, RedisClient> {
    pub async fn new(config: C) -> anyhow::Result<Self> {
        let db = PgDatabase::connect(config.database_url())
            .await
            .context("Failed to create PgDatabase")?;
        let redis = RedisClient::connect(config.redis_url())
            .await
            .context("Failed to create RedisClient")?;
        return Ok(Self { config, db, redis });
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let addr = format!("{}:{}", self.config.host(), self.config.port());
        let listener = TcpListener::bind(&addr)
            .await
            .context("Failed to start tcp connection")?;
        let router = Router::new();
        tracing::info!("Listening on http://{addr}");
        axum::serve(listener, router)
            .await
            .context("Failed to start axum server")?;
        Ok(())
    }
}
