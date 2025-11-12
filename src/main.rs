mod cli;
mod configuration;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = cli::evaluate()?;

    let mut runtime = runtime::Runtime::new()?;

    let gateway = gateway::Gateway::new(&configuration.gateway)?;
    let gateway_sender = runtime.sender();
    let api = api::API::new(&configuration.api).await?;
    let api_sender = runtime.sender();

    let mut set = tokio::task::JoinSet::new();
    set.spawn(async move { gateway.run(gateway_sender).await });
    set.spawn(async move { api.run(api_sender).await });
    runtime.start().await?;
    set.join_all().await;

    Ok(())
}
