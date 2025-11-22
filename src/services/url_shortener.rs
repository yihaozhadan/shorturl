use crate::db::{error::Error, url_mapping_repo::UrlMappingRepo};
use crate::models::url_mapping::UrlMapping;

#[derive(Clone)]
pub struct UrlShortenerService {
    repo: UrlMappingRepo,
}

impl UrlShortenerService {
    pub fn new(repo: UrlMappingRepo) -> Self {
        Self { repo }
    }

    pub async fn get_or_create(&self, long_url: &str) -> Result<UrlMapping, Error> {
        if let Some(existing) = self.repo.find_by_long_url(long_url).await? {
            return Ok(existing);
        }

        let short_code = nanoid::nanoid!(8);
        self.repo.create_mapping(long_url, &short_code).await
    }

    pub async fn get_long_url(&self, short_code: &str) -> Result<Option<String>, Error> {
        let mapping = self.repo.find_by_short_code(short_code).await?;
        if let Some(ref m) = mapping {
            // Update last_accessed_at and access_count asynchronously in DB
            self.repo.touch_by_short_code(&m.short_code).await?;
        }

        Ok(mapping.map(|m| m.long_url))
    }
}
