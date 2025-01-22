pub struct Configuration {
    pub port: u16,
}

impl Default for Configuration {
    fn default() -> Self {
        Self { port: 8150 }
    }
}
