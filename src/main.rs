mod cli;
mod configuration;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = cli::evaluate()?;
    let gateway = gateway::Gateway::new(&configuration.gateway);
    let api = api::API::new(&configuration.api);
    let mut set = tokio::task::JoinSet::new();
    set.spawn(async move { gateway.run().await });
    set.spawn(async move { api.run().await });
    set.join_all().await;
    Ok(())
}
