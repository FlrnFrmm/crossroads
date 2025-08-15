#[derive(Debug, serde::Deserialize, garde::Validate)]
pub struct Configuration {
    #[garde(alphanumeric)]
    pub name: String,
    #[garde(ascii)]
    pub path: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            name: "roads".into(),
            path: ".".into(),
        }
    }
}
