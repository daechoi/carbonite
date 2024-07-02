use std::net::TcpListener;

use actix_web::{dev::Server, get, web, App, HttpResponse, HttpServer, Responder};

#[get("/greet/{name}")]
async fn greet(info: web::Path<String>) -> impl Responder {
    format!("Hello {}!", info)
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(greet).service(health_check))
        .listen(listener)?
        .run();

    Ok(server)
}
