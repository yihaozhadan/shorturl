use actix_web::{HttpResponse, Responder, ResponseError, post, web};
use serde::Deserialize;

use crate::services::url_shortener::UrlShortenerService;
use crate::services::url_validation::{ValidationError, validate_url};

#[derive(Deserialize)]
pub struct CreateShortUrlRequest {
    pub long_url: String,
}

#[post("/shorten")]
pub async fn create_short_url(
    service: web::Data<UrlShortenerService>,
    payload: web::Json<CreateShortUrlRequest>,
) -> impl Responder {
    if let Err(err) = validate_url(&payload.long_url) {
        let (status, message, reason) = match err {
            ValidationError::Empty => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "Invalid URL",
                "URL must not be empty",
            ),
            ValidationError::InvalidFormat => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "Invalid URL",
                "URL must be a valid http or https URL",
            ),
        };

        return HttpResponse::build(status).json(serde_json::json!({
            "error": message,
            "reason": reason,
        }));
    }

    match service.get_or_create(&payload.long_url).await {
        Ok(mapping) => HttpResponse::Ok().json(mapping),
        Err(err) => err.error_response(),
    }
}
