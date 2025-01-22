mod api;
mod configuration;
mod gateway;

use anyhow::Result;
use tokio::select;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = configuration::Configuration::default();
    let gateway = gateway::Gateway::new(&configuration);
    let api = api::API::new(&configuration);
    select! {
        gateway_result = gateway.run() => {
            gateway_result
        }
        api_result = api.run() => {
            api_result
        }
    }
}
