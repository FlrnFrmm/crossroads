mod api;
mod configuration;
mod gateway;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = configuration::Configuration::default();
    let gateway = gateway::Gateway::new(&configuration);
    let api = api::API::new(&configuration);
    let mut set = tokio::task::JoinSet::new();
    set.spawn(async move { gateway.run().await });
    set.spawn(async move { api.run().await });
    set.join_all().await;
    Ok(())
}
