use expense_tracker_api::{config::AppConfig, server};
use std::process;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::new().unwrap_or_else(|err| {
        eprintln!("Failed to initialize AppConfig: {}", err);
        process::exit(1);
    });
    server::setup_server(config).await?;
    return Ok(());
}
