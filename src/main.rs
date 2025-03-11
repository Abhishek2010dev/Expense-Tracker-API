use anyhow::Context;
use dotenv::dotenv;
use expense_tracker_api::{config::AppConfig, server};
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().context("Failed to load .env")?;
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .with_span_events(FmtSpan::NONE)
        .init();

    let config = AppConfig::new().context("Failed to initialize AppConfig")?;

    server::setup_server(config).await?;
    return Ok(());
}
