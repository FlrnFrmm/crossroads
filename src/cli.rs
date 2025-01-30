use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use crate::configuration::Configuration;

pub fn evaluate() -> Result<Configuration> {
    let cli = Cli::parse();
    if let Some(path) = cli.configuration.as_deref() {
        return Configuration::from_configuration_file(path);
    }
    Configuration::default()
}

#[derive(Parser)]
#[command(version, about = "Your blazingly fast API Gateway", long_about = None)]
struct Cli {
    /// Path to the configuration file
    #[arg(short, long, value_name = "PATH")]
    configuration: Option<PathBuf>,
}
