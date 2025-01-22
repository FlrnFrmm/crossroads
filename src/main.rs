mod api;
mod gateway;

use anyhow::Result;
use tokio::select;

#[tokio::main]
async fn main() -> Result<()> {
    select! {
        gateway_result = gateway::run() => {
            gateway_result
        }
        api_result = api::run() => {
            api_result
        }
    }
}
