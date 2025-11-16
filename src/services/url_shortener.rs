use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::models::url_mapping::UrlMapping;

#[derive(Clone, Default)]
pub struct InMemoryUrlStore {
    inner: Arc<RwLock<HashMap<String, String>>>,
}

impl InMemoryUrlStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_create(&self, long_url: &str) -> UrlMapping {
        let mut map = self.inner.write().expect("lock poisoned");

        if let Some(existing) = map.iter().find(|(_, v)| v.as_str() == long_url) {
            return UrlMapping {
                short_code: existing.0.clone(),
                long_url: existing.1.clone(),
            };
        }

        let short_code = nanoid::nanoid!(8);
        map.insert(short_code.clone(), long_url.to_string());

        UrlMapping {
            short_code,
            long_url: long_url.to_string(),
        }
    }
}
