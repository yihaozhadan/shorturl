use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlMappingRecord {
    pub long_url: String,
    pub short_code: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub last_accessed_at: String,
    #[serde(default)]
    pub access_count: u64,
}
