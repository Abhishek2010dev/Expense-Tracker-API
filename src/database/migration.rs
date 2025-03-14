use async_trait::async_trait;
use sqlx::{Database, Pool};

#[async_trait()]
pub trait Migrator<DB: Database> {
    async fn migrate(&self, pool: &Pool<DB>) -> anyhow::Result<()>;
}
