use anyhow::Result;
use axum::{routing::get, Router};

pub async fn run() -> Result<()> {
    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8150").await?;
    axum::serve(listener, app).await.map_err(|err| err.into())
}

async fn root() -> &'static str {
    "Hello from the api!"
}
