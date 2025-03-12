use anyhow::{Context, Ok};
use axum::Router;
use tokio::net::TcpListener;

use crate::config::Config;

pub struct Server<C: Config> {
    config: C,
}

impl<C: Config> Server<C> {
    pub fn new(config: C) -> Self {
        return Self { config };
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let addr = format!("{}:{}", self.config.host(), self.config.port());
        let listener = TcpListener::bind(&addr)
            .await
            .context("Failed to start tcp connection")?;
        let router = Router::new();
        tracing::info!("Listening on http://{addr}");
        axum::serve(listener, router)
            .await
            .context("Failed to start axum server")?;
        Ok(())
    }
}
