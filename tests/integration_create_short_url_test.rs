use actix_web::{http::StatusCode, test, web, App};

use shorturl::services::url_shortener::InMemoryUrlStore;
use shorturl::routes::create_short_url::create_short_url;
use shorturl::routes::redirect::redirect_short_url;

#[actix_web::test]
async fn create_and_redirect_short_url_works() {
    let store = InMemoryUrlStore::new();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(store.clone()))
            .service(create_short_url)
            .service(redirect_short_url),
    )
    .await;

    let long_url = "https://example.com/test";

    let req = test::TestRequest::post()
        .uri("/shorten")
        .set_json(&serde_json::json!({ "long_url": long_url }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body_bytes = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let code = json["short_code"].as_str().unwrap();

    let req_redirect = test::TestRequest::get()
        .uri(&format!("/{}", code))
        .to_request();

    let resp_redirect = test::call_service(&app, req_redirect).await;
    assert_eq!(resp_redirect.status(), StatusCode::FOUND);
    let location = resp_redirect.headers().get("Location").unwrap();
    assert_eq!(location.to_str().unwrap(), long_url);
}
