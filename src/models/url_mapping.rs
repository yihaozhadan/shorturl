use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlMapping {
    pub short_code: String,
    pub long_url: String,
    pub created_at: String,
    pub last_accessed_at: Option<String>,
    pub access_count: u64,
}
