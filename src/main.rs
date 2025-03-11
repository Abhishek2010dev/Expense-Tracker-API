use expense_tracker_api::{config::AppConfig, server};
use std::process;
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let config = AppConfig::new().unwrap_or_else(|err| {
        tracing::error!("Failed to initialize AppConfig: {}", err);
        process::exit(1);
    });

    server::setup_server(config).await?;
    return Ok(());
}
