use actix_web::{get, web, HttpResponse, Responder};

use crate::services::url_shortener::InMemoryUrlStore;

#[get("/{code}")]
pub async fn redirect_short_url(
    store: web::Data<InMemoryUrlStore>,
    path: web::Path<String>,
) -> impl Responder {
    let code = path.into_inner();

    if let Some(long_url) = store.get_long_url(&code) {
        HttpResponse::Found()
            .append_header(("Location", long_url.as_str()))
            .finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
