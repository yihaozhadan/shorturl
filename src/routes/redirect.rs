use actix_web::{HttpResponse, Responder, ResponseError, get, web};

use crate::services::url_shortener::UrlShortenerService;

#[get("/{code}")]
pub async fn redirect_short_url(
    service: web::Data<UrlShortenerService>,
    path: web::Path<String>,
) -> impl Responder {
    let code = path.into_inner();

    match service.get_long_url(&code).await {
        Ok(Some(long_url)) => HttpResponse::Found()
            .append_header(("Location", long_url))
            .finish(),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(err) => err.error_response(),
    }
}
