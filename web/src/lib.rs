use serde::Deserialize;
use std::net::TcpListener;

use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/greet/{name}")]
async fn greet(info: web::Path<String>) -> impl Responder {
    format!("Hello {}!", info)
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[post("subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
