use std::env;

pub struct AppConfig {
    pub server_addr: String,
    pub surrealdb_url: String,
    pub surrealdb_user: String,
    pub surrealdb_pass: String,
    pub surrealdb_ns: String,
    pub surrealdb_db: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let server_addr =
            env::var("SHORTURL_SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let surrealdb_url =
            env::var("SURREALDB_URL").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
        let surrealdb_user = 
            env::var("SURREALDB_USER").unwrap_or_else(|_| "root".to_string());
        let surrealdb_pass =
            env::var("SURREALDB_PASS").unwrap_or_else(|_| "root".to_string());
        let surrealdb_ns =
            env::var("SURREALDB_NS").unwrap_or_else(|_| "shorturl".to_string());
        let surrealdb_db =
            env::var("SURREALDB_DB").unwrap_or_else(|_| "shorturl".to_string());

        Self {
            server_addr,
            surrealdb_url,
            surrealdb_user,
            surrealdb_pass,
            surrealdb_ns,
            surrealdb_db,
        }
    }
}
