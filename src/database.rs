use anyhow::Result;
use sqlx::{Database, Pool};

pub trait DatabaseConnection<DB: Database> {
    async fn connect(database_url: &str) -> Result<Self>
    where
        Self: Sized;

    fn pool(&self) -> Pool<DB>;
}
