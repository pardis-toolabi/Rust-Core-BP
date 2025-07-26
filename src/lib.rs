use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}

#[get("/healthCheck")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(Serialize, Deserialize)]
struct FormData{
    name: String,
    email: String
}
#[post("/subscriptions")]
async fn subscriptions(web::Form(form): web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

