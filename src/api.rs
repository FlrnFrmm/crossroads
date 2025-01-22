use anyhow::Result;
use axum::{routing::get, Router};

use crate::configuration::Configuration;

pub struct API {
    port: u16,
}

impl API {
    pub fn new(configuration: &Configuration) -> Self {
        Self {
            port: configuration.api.port,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let app = Router::new().route("/", get(root));
        let address = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(address).await?;
        axum::serve(listener, app).await.map_err(|err| err.into())
    }
}

async fn root() -> &'static str {
    "Hello from the api!"
}
