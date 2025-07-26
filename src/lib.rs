use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, get};

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
