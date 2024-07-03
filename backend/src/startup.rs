use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, conn_pool: PgPool) -> Result<Server, std::io::Error> {
    let conn_pool = web::Data::new(conn_pool);
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(subscribe)
            .app_data(conn_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
