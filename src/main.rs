use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/healthCheck")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
