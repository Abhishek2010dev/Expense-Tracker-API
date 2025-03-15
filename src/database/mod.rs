pub mod migration;

use std::time::Duration;

use anyhow::{Context, Result};
use async_trait::async_trait;
use migration::Migrator;
use sqlx::{Database, Pool, Postgres, postgres::PgPoolOptions};

#[async_trait]
pub trait DatabaseConnection<DB: Database> {
    async fn connect(database_url: &str, migrator: &dyn Migrator<DB>) -> Result<Self>
    where
        Self: Sized;

    fn pool(&self) -> &Pool<DB>;
}

pub struct PgDatabase(Pool<Postgres>);

#[async_trait]
impl DatabaseConnection<Postgres> for PgDatabase {
    async fn connect(database_url: &str, migrator: &dyn Migrator<Postgres>) -> Result<Self>
    where
        Self: Sized,
    {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(database_url)
            .await
            .context("can't connect to database")?;
        tracing::info!("Successfully connected to PostgreSQL");

        migrator.migrate(&pool).await?;

        Ok(Self(pool))
    }

    fn pool(&self) -> &Pool<Postgres> {
        &self.0
    }
}

impl Drop for PgDatabase {
    fn drop(&mut self) {
        if self.0.is_closed() {
            tracing::info!("PostgreSQL connection was already closed.");
        } else {
            tracing::info!("Closing PostgreSQL connection.");
        }
    }
}
