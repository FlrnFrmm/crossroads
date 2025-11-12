use chrono::Utc;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProxyMetadata {
    pub tag: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl ProxyMetadata {
    pub fn new(tag: String) -> Self {
        Self {
            tag,
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
        }
    }
}
