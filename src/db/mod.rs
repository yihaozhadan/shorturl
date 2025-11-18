pub mod error;
pub mod schema;
pub mod url_mapping_repo;

use crate::config::env::AppConfig;
use crate::db::error::Error;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client as WsClient, Ws};
use surrealdb::opt::auth::Root;

type SurrealConnection = Surreal<WsClient>;

#[derive(Clone)]
pub struct DbClient {
    inner: SurrealConnection,
}

impl DbClient {
    fn new(inner: SurrealConnection) -> Self {
        Self { inner }
    }

    pub fn connection(&self) -> &SurrealConnection {
        &self.inner
    }
}

pub async fn create_db_client(cfg: &AppConfig) -> Result<DbClient, Error> {
    let db = Surreal::new::<Ws>(&cfg.surrealdb_url).await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("shorturl").use_db("shorturl").await?;

    Ok(DbClient::new(db))
}
