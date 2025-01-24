use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;

use crate::configuration::Configuration;

pub fn evaluate() -> Result<Configuration> {
    let cli = Cli::parse();

    let mut configuration = Configuration::default();

    if let Some(path) = cli.configuration.as_deref() {
        let _file_content = std::fs::read_to_string(path)
            .map_err(|err| anyhow!("\nCouldn't read \'{:?}\'\n{}", path, err))?;
        // TODO: Parse the file content
    }

    if let Some(port) = cli.api_port {
        configuration.api.port = port;
    }

    if let Some(port) = cli.gateway_port {
        configuration.gateway.port = port;
    }

    Ok(configuration)
}

#[derive(Parser)]
#[command(version, about = "Your blazingly fast API Gateway", long_about = None)]
struct Cli {
    /// Path to the configuration file
    #[arg(short, long, value_name = "PATH")]
    configuration: Option<PathBuf>,
    /// Port for the API server
    #[arg(short, long, value_name = "PORT")]
    api_port: Option<u16>,
    /// Port for the Gateway server
    #[arg(short, long, value_name = "PORT")]
    gateway_port: Option<u16>,
}
