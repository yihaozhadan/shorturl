use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlMappingRecord {
    pub id: Option<String>,
    pub long_url: String,
    pub short_code: String,
    pub created_at: String,
    pub last_accessed_at: Option<String>,
}
