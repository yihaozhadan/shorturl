use crate::db::{DbClient, error::Error, schema::UrlMappingRecord};
use crate::models::url_mapping::UrlMapping;
use serde_json::json;

const TABLE: &str = "url_mappings";

#[derive(Clone)]
pub struct UrlMappingRepo {
    db: DbClient,
}

impl UrlMappingRepo {
    pub fn new(db: DbClient) -> Self {
        Self { db }
    }

    pub async fn find_by_long_url(&self, long_url: &str) -> Result<Option<UrlMapping>, Error> {
        let long_url_value = long_url.to_owned();
        let mut response = self
            .db
            .connection()
            .query("SELECT * FROM type::table($table) WHERE long_url = $long_url LIMIT 1")
            .bind(("table", TABLE))
            .bind(("long_url", long_url_value))
            .await?;

        let record: Option<UrlMappingRecord> = response.take(0)?;
        Ok(record.map(Self::to_model))
    }

    pub async fn touch_by_short_code(&self, short_code: &str) -> Result<(), Error> {
        let short_code_value = short_code.to_owned();
        self
            .db
            .connection()
            .query(
                "UPDATE type::table($table) SET last_accessed_at = time::now(), access_count += 1 WHERE short_code = $short_code",
            )
            .bind(("table", TABLE))
            .bind(("short_code", short_code_value))
            .await?;

        Ok(())
    }

    pub async fn find_by_short_code(&self, short_code: &str) -> Result<Option<UrlMapping>, Error> {
        let short_code_value = short_code.to_owned();
        let mut response = self
            .db
            .connection()
            .query("SELECT * FROM type::table($table) WHERE short_code = $short_code LIMIT 1")
            .bind(("table", TABLE))
            .bind(("short_code", short_code_value))
            .await?;

        let record: Option<UrlMappingRecord> = response.take(0)?;
        Ok(record.map(Self::to_model))
    }

    pub async fn create_mapping(
        &self,
        long_url: &str,
        short_code: &str,
    ) -> Result<UrlMapping, Error> {
        let long_url_value = long_url.to_owned();
        let short_code_value = short_code.to_owned();
        let mut response = self
            .db
            .connection()
            .query("CREATE type::table($table) CONTENT $content")
            .bind(("table", TABLE))
            .bind((
                "content",
                json!({
                    "long_url": long_url_value,
                    "short_code": short_code_value,
                }),
            ))
            .await?;

        let record: Option<UrlMappingRecord> = response.take(0)?;
        record.map(Self::to_model).ok_or(Error::Db)
    }

    fn to_model(record: UrlMappingRecord) -> UrlMapping {
        UrlMapping {
            short_code: record.short_code,
            long_url: record.long_url,
            created_at: record.created_at,
            last_accessed_at: if record.last_accessed_at.is_empty() {
                None
            } else {
                Some(record.last_accessed_at)
            },
            access_count: record.access_count,
        }
    }
}
