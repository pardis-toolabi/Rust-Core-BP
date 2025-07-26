use actix_web::{App, http::header::ContentType, test};
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
