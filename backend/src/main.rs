use carbonite::configuration::Settings;
use carbonite::startup::run;
use carbonite::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("carbonite".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = Settings::from_file("config/config.yaml").expect("failed to read configuration");
    let conn_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.application_port))
        .expect("Failed to bind to 8080");
    run(listener, conn_pool)?.await
}
