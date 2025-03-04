mod api;
mod gateway;

use anyhow::{anyhow, Result};
use kcl_lang::{ExecProgramArgs, API};
use serde::{Deserialize, Serialize};
use std::path::Path;

static SCHEMA: &str = include_str!("../configuration/schema.k");

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub api: api::Configuration,
    pub gateway: gateway::Configuration,
}

impl Configuration {
    pub fn default() -> Result<Self> {
        Self::process_kcl(None)
    }

    pub fn from_configuration_file(path: &Path) -> Result<Self> {
        Self::process_kcl(Some(path))
    }

    fn process_kcl(path: Option<&Path>) -> Result<Self> {
        let mut configuration_file_name = "default-configuration.k".to_string();
        let mut code = String::new();
        if let Some(path) = path {
            configuration_file_name = "configuration.k".to_string();
            code = std::fs::read_to_string(path)?;
        }
        let code = format!("{}\nConfiguration{{\n{}}}", SCHEMA, code);
        let args = &ExecProgramArgs {
            k_filename_list: vec![configuration_file_name],
            k_code_list: vec![code],
            ..Default::default()
        };
        let api = API::default();
        let exec_result = api.exec_program(args)?;
        if !exec_result.err_message.is_empty() {
            return Err(anyhow!(
                "Configuration error:\n\n{}",
                exec_result.err_message
            ));
        }
        serde_json::from_str(&exec_result.json_result).map_err(|err| anyhow!(err))
    }
}
