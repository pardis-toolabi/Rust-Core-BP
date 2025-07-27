use actix_web::{
    App,
    http::{self, header::ContentType},
    test,
};
use rust_core_bp::*;

#[actix_web::test]
async fn health_check_test() {
    let app = test::init_service(App::new().service(health_check)).await;
    let req = test::TestRequest::get()
        .uri("/healthCheck")
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{:?}", resp);
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn subscription_test() {
    let app = test::init_service(App::new().service(subscriptions)).await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let req = test::TestRequest::post()
        .uri("/subscriptions")
        .insert_header((
            http::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        ))
        .set_payload(body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{:?}", resp);
    assert!(resp.status().is_success());
}
