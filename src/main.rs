use anyhow::Context;
use dotenv::dotenv;
use expense_tracker_api::{
    config::{env_config::EnvConfig, env_provider::StdEnv},
    server::Server,
};
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

    let config = EnvConfig::new(StdEnv).context("Failed to initialize AppConfig")?;

    let server = Server::new(config);
    server.run().await?;
    return Ok(());
}
