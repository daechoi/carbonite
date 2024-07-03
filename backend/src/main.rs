use std::net::TcpListener;

use carbonite::configuration::Settings;
use carbonite::startup::run;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::from_file("config/config.yaml").expect("failed to read configuration");
    let conn_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.application_port))
        .expect("Failed to bind to 8080");
    run(listener, conn_pool)?.await
}
