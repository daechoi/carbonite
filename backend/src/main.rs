use std::net::TcpListener;

use carbonite::configuration::Settings;
use carbonite::startup::run;
use env_logger::Env;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // `init` does call `set_logger`, so this is all we need to do.
    // we are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable is not set.

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let settings = Settings::from_file("config/config.yaml").expect("failed to read configuration");
    let conn_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.application_port))
        .expect("Failed to bind to 8080");
    run(listener, conn_pool)?.await
}
