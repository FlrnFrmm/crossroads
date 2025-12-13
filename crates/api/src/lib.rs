pub mod configuration;
pub mod database;
mod endpoints;
mod error;

use anyhow::Result;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::database::Database;
use configuration::Configuration;
use runtime::Runtime;

pub struct API {
    port: u16,
    database: Database,
}

impl API {
    pub async fn new(configuration: &Configuration) -> Result<Self> {
        let api = Self {
            port: configuration.port,
            database: Database::new(&configuration.database).await?,
        };
        Ok(api)
    }

    pub async fn run(self, runtime: Runtime) -> Result<()> {
        let app = Router::new()
            .route("/proxies/current", get(endpoints::current_proxy))
            .route("/proxies/current/{tag}", get(endpoints::set_current_proxy))
            .route("/proxies", get(endpoints::all_proxies))
            .route("/proxies/{tag}", post(endpoints::create_proxy))
            .route("/proxies/{tag}", get(endpoints::get_proxy))
            .route("/proxies/{tag}", put(endpoints::update_proxy))
            .route("/proxies/{tag}", delete(endpoints::delete_proxy))
            .with_state((Arc::new(RwLock::new(self.database)), runtime));
        let address = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(address).await?;
        axum::serve(listener, app).await.map_err(|err| err.into())
    }
}
