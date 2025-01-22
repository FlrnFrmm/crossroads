mod api;
mod gateway;

pub struct Configuration {
    pub api: api::Configuration,
    pub gateway: gateway::Configuration,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            api: api::Configuration::default(),
            gateway: gateway::Configuration::default(),
        }
    }
}
