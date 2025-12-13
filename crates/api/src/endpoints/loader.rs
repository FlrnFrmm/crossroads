use anyhow::Result;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Loader {
    #[serde(rename = "payload")]
    Payload(Vec<u8>),
    #[serde(rename = "registryCredentials")]
    Registry(RegistryCredentials), // Todo
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
            Loader::Payload(bytes) => Ok(bytes),
            Loader::Registry(_registry_credentials) => todo!(),
        }
    }
}
