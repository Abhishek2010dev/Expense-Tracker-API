use anyhow::Context;
use dotenv::dotenv;
use expense_tracker_api::{config::AppConfig, server};
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().context("Failed to load .env")?;
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let config = AppConfig::new().context("Failed to initialize AppConfig")?;

    server::setup_server(config).await?;
    return Ok(());
}
