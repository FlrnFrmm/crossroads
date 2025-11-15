mod cli;
mod configuration;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = cli::evaluate()?;

    let mut handler = runtime::Handler::new()?;

    let gateway = gateway::Gateway::new(&configuration.gateway)?;
    let gateway_sender = handler.sender.clone();
    let api = api::API::new(&configuration.api).await?;
    let api_sender = handler.sender.clone();

    let mut set = tokio::task::JoinSet::new();
    set.spawn(async move { gateway.run(gateway_sender).await });
    set.spawn(async move { api.run(api_sender).await });
    handler.start().await?;
    set.join_all().await;

    Ok(())
}
