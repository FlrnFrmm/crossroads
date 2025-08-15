pub mod configuration;
pub mod database;
mod endpoints;
mod errors;
pub mod road;

use anyhow::Result;
use axum::routing::{delete, get, post, put};
use axum::Router;

use configuration::Configuration;

pub struct API {
    port: u16,
}

impl API {
    pub fn new(configuration: &Configuration) -> Self {
        Self {
            port: configuration.port,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let roads = endpoints::Roads::default();
        let app = Router::new()
            .route("/roads", post(endpoints::create_road))
            .route("/roads", get(endpoints::all_roads))
            .route("/roads/{host}", get(endpoints::get_road))
            .route("/roads/{host}", put(endpoints::update_road))
            .route("/roads/{host}", delete(endpoints::delete_road))
            .with_state(roads);
        let address = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(address).await?;
        axum::serve(listener, app).await.map_err(|err| err.into())
    }
}
