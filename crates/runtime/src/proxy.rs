mod metadata;

pub use metadata::ProxyMetadata;

#[derive(Debug)]
pub struct Proxy {
    pub metadata: ProxyMetadata,
    pub component: Vec<u8>,
}

impl Proxy {
    pub fn new(tag: String, component: Vec<u8>) -> Self {
        Self {
            metadata: ProxyMetadata::new(tag),
            component,
        }
    }
}
