use std::path::PathBuf;

use super::validation;

#[derive(Debug, serde::Deserialize, garde::Validate)]
pub struct Configuration {
    #[garde(alphanumeric)]
    pub tag: String,
    #[garde(custom(validation::is_valid_path))]
    pub path: PathBuf,
}
