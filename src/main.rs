mod cli;
mod configuration;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = cli::evaluate()?;

    let wasm_bytes = include_bytes!("../target/wasm32-wasip2/release/proxy.wasm");
    let runtime = runtime::Runtime::new(wasm_bytes)?;

    let gateway = gateway::Gateway::new(&configuration.gateway)?;
    let api = api::API::new(&configuration.api).await?;

    let mut set = tokio::task::JoinSet::new();
    let gateway_runtime = runtime.clone();
    set.spawn(async move { gateway.run(gateway_runtime).await });
    let api_runtime = runtime.clone();
    set.spawn(async move { api.run(api_runtime).await });

    set.join_all().await;

    Ok(())
}
