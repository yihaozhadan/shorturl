use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlMapping {
    pub short_code: String,
    pub long_url: String,
}
