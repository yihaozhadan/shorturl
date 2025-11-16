use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod config;
mod db;
mod models;
mod routes;
mod services;

use crate::config::env::AppConfig;
use crate::config::logging::init_logging;
use crate::services::url_shortener::InMemoryUrlStore;

#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("shorturl backend is running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging();
    let cfg = AppConfig::from_env();

    log::info!("Starting shorturl backend on {}", cfg.server_addr);

    let store = InMemoryUrlStore::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(store.clone()))
            .service(health)
            .service(routes::create_short_url::create_short_url)
            .service(routes::redirect::redirect_short_url)
    })
    .bind(&cfg.server_addr)?
    .run()
    .await
}
