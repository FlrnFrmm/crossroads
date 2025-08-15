use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Road {
    pub host: String,
    pub component: Vec<u8>,
}
