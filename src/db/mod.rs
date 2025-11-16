use crate::config::env::AppConfig;

#[derive(Clone, Debug, Default)]
pub struct DbClient;

pub async fn create_db_client(_cfg: &AppConfig) -> Result<DbClient, ()> {
    // TODO: Implement real SurrealDB connection once we finalize the v2 client usage.
    Ok(DbClient::default())
}