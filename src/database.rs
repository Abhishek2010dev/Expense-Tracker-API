use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use sqlx::{Database, Pool, Postgres, postgres::PgPoolOptions};

pub trait DatabaseConnection<DB: Database> {
    async fn connect(database_url: &str) -> Result<Self>
    where
        Self: Sized;

    fn pool(&self) -> &Pool<DB>;
}

pub struct PgDatabase(Arc<Pool<Postgres>>);

impl DatabaseConnection<Postgres> for PgDatabase {
    async fn connect(database_url: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(database_url)
            .await
            .context("can't connect to database")?;
        Ok(Self(Arc::new(pool)))
    }

    fn pool(&self) -> &Pool<Postgres> {
        &self.0
    }
}
