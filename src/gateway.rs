use anyhow::{anyhow, Result};
use rama::http::response::Json;
use rama::http::server::HttpServer;
use rama::http::Request;
use rama::net::address::SocketAddress;
use rama::rt::Executor;
use rama::service::service_fn;
use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr};

use crate::configuration::Configuration;

pub struct Gateway {
    port: u16,
}

impl Gateway {
    pub fn new(configuration: &Configuration) -> Self {
        Self {
            port: configuration.gateway.port,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let address = SocketAddress::new(ip, self.port);
        let service = service_fn(handle);
        HttpServer::auto(Executor::default())
            .listen(address, service)
            .await
            .map_err(|error| anyhow!("{}", error.to_string()))
    }
}

async fn handle(request: Request) -> Result<Json<serde_json::Value>, Infallible> {
    let data = serde_json::json!({
        "method": request.method().as_str(),
        "path": request.uri().path(),
    });
    let response = Json(data);
    Ok(response)
}
