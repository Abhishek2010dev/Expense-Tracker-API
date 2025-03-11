use anyhow::Context;
use expense_tracker_api::{config::AppConfig, server};
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let config = AppConfig::new().context("Failed to initialize AppConfig")?;

    server::setup_server(config).await?;
    return Ok(());
}
