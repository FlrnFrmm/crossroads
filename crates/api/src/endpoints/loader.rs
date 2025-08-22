use anyhow::{anyhow, Result};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Loader {
    Path(PathBuf),
    Payload(Vec<u8>),
    Registry(RegistryCredentials),
}

#[derive(Serialize, Deserialize)]
pub struct RegistryCredentials {
    host: String,
    login: Option<Login>,
    tag: String,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

impl Loader {
    pub fn load(self) -> Result<Vec<u8>> {
        match self {
            Loader::Path(path) => {
                std::fs::read(path).map_err(|e| anyhow!("Couldn't load file: {}", e))
            }
            Loader::Payload(bytes) => Ok(bytes),
            Loader::Registry(_registry_credentials) => todo!(),
        }
    }
}
