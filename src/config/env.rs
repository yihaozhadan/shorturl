use std::env;

pub struct AppConfig {
    pub server_addr: String,
    pub surrealdb_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let server_addr = env::var("SHORTURL_SERVER_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let surrealdb_url = env::var("SURREALDB_URL")
            .unwrap_or_else(|_| "ws://127.0.0.1:8000/rpc".to_string());

        Self {
            server_addr,
            surrealdb_url,
        }
    }
}
