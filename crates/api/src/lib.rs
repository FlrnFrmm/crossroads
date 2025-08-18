pub mod configuration;
pub mod database;
mod endpoints;
mod errors;
pub mod road;

use std::sync::Arc;
use tokio::sync::RwLock;

use anyhow::Result;
use axum::routing::{delete, get, post, put};
use axum::Router;

use configuration::Configuration;

use crate::database::Database;

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

    pub async fn run(self) -> Result<()> {
        let app = Router::new()
            .route("/roads", get(endpoints::all_roads))
            .route("/roads/{host}", post(endpoints::create_road))
            .route("/roads/{host}", get(endpoints::get_road))
            .route("/roads/{host}", put(endpoints::update_road))
            .route("/roads/{host}", delete(endpoints::delete_road))
            .with_state(Arc::new(RwLock::new(self.database)));
        let address = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(address).await?;
        axum::serve(listener, app).await.map_err(|err| err.into())
    }
}
