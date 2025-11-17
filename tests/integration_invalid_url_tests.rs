use actix_web::{http::StatusCode, test, web, App};

use shorturl::routes::create_short_url::create_short_url;
use shorturl::services::url_shortener::InMemoryUrlStore;

#[actix_web::test]
async fn empty_url_is_rejected() {
    let store = InMemoryUrlStore::new();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(store.clone()))
            .service(create_short_url),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/shorten")
        .set_json(&serde_json::json!({ "long_url": "" }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let body_bytes = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json["error"].as_str().unwrap(), "Invalid URL");
}

#[actix_web::test]
async fn malformed_url_is_rejected() {
    let store = InMemoryUrlStore::new();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(store.clone()))
            .service(create_short_url),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/shorten")
        .set_json(&serde_json::json!({ "long_url": "not a url" }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
