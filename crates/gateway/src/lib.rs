pub mod configuration;
mod proxy;

use anyhow::{Error, Result};
use rama::{http::server::HttpServer, rt::Executor};
use tokio::sync::mpsc::UnboundedSender;

use configuration::Configuration;

use runtime::Message;

pub struct Gateway {
    port: u16,
}

impl Gateway {
    pub fn new(configuration: &Configuration) -> Result<Self> {
        let gateway = Self {
            port: configuration.port,
        };
        Ok(gateway)
    }

    pub async fn run(&self, sender: UnboundedSender<Message>) -> Result<()> {
        let executor = Executor::default();
        let address = ([0, 0, 0, 0], self.port);
        let proxy = proxy::WebAssemblyComponentProxy::new(sender);
        HttpServer::auto(executor)
            .listen(address, proxy)
            .await
            .map_err(|e| Error::from_boxed(e))
    }
}
