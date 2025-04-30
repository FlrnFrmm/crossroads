mod api;
mod gateway;
mod validation;

use anyhow::Result;
use garde::Validate;
use std::path::Path;

#[derive(Debug, serde::Deserialize, garde::Validate)]
pub struct Configuration {
    #[garde(dive)]
    #[serde(default)]
    pub api: api::Configuration,
    #[garde(dive)]
    #[serde(default)]
    pub gateway: gateway::Configuration,
}

impl Configuration {
    pub fn from_configuration_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let configuration: Configuration = serde_yaml::from_str(&content)?;
        configuration.validate()?;
        Ok(configuration)
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            api: api::Configuration::default(),
            gateway: gateway::Configuration::default(),
        }
    }
}
