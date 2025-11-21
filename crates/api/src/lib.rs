pub mod configuration;
pub mod database;
mod endpoints;
mod error;

use anyhow::Result;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::mpsc::UnboundedSender;

use crate::database::Database;
use configuration::Configuration;
use runtime::Message;

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

    pub async fn run(self, sender: UnboundedSender<Message>) -> Result<()> {
        let app = Router::new()
            .route("/proxys/current", get(endpoints::current_proxy))
            .route("/proxys/current/{tag}", get(endpoints::set_current_proxy))
            .route("/proxys", get(endpoints::all_proxies))
            .route("/proxys/{tag}", post(endpoints::create_proxy))
            .route("/proxys/{tag}", get(endpoints::get_proxy))
            .route("/proxys/{tag}", put(endpoints::update_proxy))
            .route("/proxys/{tag}", delete(endpoints::delete_proxy))
            .with_state((Arc::new(RwLock::new(self.database)), sender));
        let address = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(address).await?;
        axum::serve(listener, app).await.map_err(|err| err.into())
    }
}
