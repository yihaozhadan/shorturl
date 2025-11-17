use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::services::url_shortener::InMemoryUrlStore;
use crate::services::url_validation::{validate_url, ValidationError};

#[derive(Deserialize)]
pub struct CreateShortUrlRequest {
    pub long_url: String,
}

#[post("/shorten")]
pub async fn create_short_url(
    store: web::Data<InMemoryUrlStore>,
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

    let mapping = store.get_or_create(&payload.long_url);
    HttpResponse::Ok().json(mapping)
}
