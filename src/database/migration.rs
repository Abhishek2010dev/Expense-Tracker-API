use anyhow::Context;
use async_trait::async_trait;
use sqlx::{Database, Pool, Postgres};

#[async_trait]
pub trait Migrator<DB: Database>: Send + Sync {
    async fn migrate(&self, pool: &Pool<DB>) -> anyhow::Result<()>;
}

pub struct PgMigrator;

#[async_trait]
impl Migrator<Postgres> for PgMigrator {
    async fn migrate(&self, pool: &Pool<Postgres>) -> anyhow::Result<()> {
        sqlx::migrate!()
            .run(pool)
            .await
            .context("Failed to run migration")
    }
}
