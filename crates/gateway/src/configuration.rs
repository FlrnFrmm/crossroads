mod validation;

#[derive(Debug, serde::Deserialize, garde::Validate)]
pub struct Configuration {
    #[garde(custom(validation::is_valid_port))]
    #[serde(default)]
    pub port: u16,
}

impl Default for Configuration {
    fn default() -> Self {
        Self { port: 80 }
    }
}
