use anyhow::Result;
use garde::Validate;
use std::path::Path;

#[derive(Debug, serde::Deserialize, garde::Validate, Default)]
pub struct Configuration {
    #[garde(dive)]
    #[serde(default)]
    pub api: api::configuration::Configuration,
    #[garde(dive)]
    #[serde(default)]
    pub gateway: gateway::configuration::Configuration,
}

impl Configuration {
    pub fn from_configuration_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let configuration: Configuration = serde_yaml::from_str(&content)?;
        configuration.validate()?;
        Ok(configuration)
    }
}
