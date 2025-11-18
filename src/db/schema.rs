use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlMappingRecord {
    pub long_url: String,
    pub short_code: String,
}
