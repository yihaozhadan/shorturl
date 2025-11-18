use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

mod config;
mod db;
mod models;
mod routes;
mod services;

use crate::config::env::AppConfig;
use crate::config::logging::init_logging;
use crate::db::DbClient;
use crate::db::url_mapping_repo::UrlMappingRepo;
use crate::services::url_shortener::UrlShortenerService;

#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("shorturl backend is running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging();
    let cfg = AppConfig::from_env();

    log::info!("Starting shorturl backend on {}", cfg.server_addr);

    let db_client: DbClient = match db::create_db_client(&cfg).await {
        Ok(client) => client,
        Err(err) => {
            log::error!("Failed to connect to SurrealDB: {err}");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "db init failed",
            ));
        }
    };
    let service = UrlShortenerService::new(UrlMappingRepo::new(db_client));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service.clone()))
            .service(health)
            .service(routes::create_short_url::create_short_url)
            .service(routes::redirect::redirect_short_url)
    })
    .bind(&cfg.server_addr)?
    .run()
    .await
}
