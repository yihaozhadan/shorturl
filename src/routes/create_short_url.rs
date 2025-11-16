use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::services::url_shortener::InMemoryUrlStore;

#[derive(Deserialize)]
pub struct CreateShortUrlRequest {
    pub long_url: String,
}

#[post("/shorten")]
pub async fn create_short_url(
    store: web::Data<InMemoryUrlStore>,
    payload: web::Json<CreateShortUrlRequest>,
) -> impl Responder {
    let mapping = store.get_or_create(&payload.long_url);
    HttpResponse::Ok().json(mapping)
}
