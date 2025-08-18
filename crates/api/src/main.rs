use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Default::default();
    let api = api::API::new(&configuration).await?;
    api.run().await
}
