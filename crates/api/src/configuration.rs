pub mod database;
mod validation;

#[derive(Debug, serde::Deserialize, garde::Validate)]
pub struct Configuration {
    #[garde(custom(validation::is_valid_port))]
    #[serde(default)]
    pub port: u16,
    #[garde(dive)]
    pub database: database::Configuration,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            port: 8150,
            database: Default::default(),
        }
    }
}
