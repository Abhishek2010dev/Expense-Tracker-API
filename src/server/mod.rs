mod routes;

use anyhow::{Context, Ok};
use routes::setup_routes;
use tokio::net::TcpListener;

use crate::config::AppConfig;

pub async fn setup_server(config: AppConfig) -> anyhow::Result<()> {
    let addr = format!("localhost:{}", config.port());
    let listener = TcpListener::bind(&addr)
        .await
        .context("Failed to start tcp connection")?;
    let routes = setup_routes();
    tracing::info!("Listening on http://{addr}");
    axum::serve(listener, routes)
        .await
        .context("Failed to start axum server")?;
    Ok(())
}
