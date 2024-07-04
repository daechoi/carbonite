use carbonite::configuration::Settings;
use carbonite::startup::run;
use carbonite::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("carbonite".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = Settings::from_file().expect("failed to read configuration");
    let conn_pool = PgPoolOptions::new()
        .idle_timeout(Duration::from_secs(2))
        .connect_lazy(&settings.database.connection_string().expose_secret())
        .expect("Failed to connect to database");

    let listener = TcpListener::bind(format!(
        "{}:{}",
        settings.application.host, settings.application.port
    ))
    .expect("Failed to bind to 8080");
    run(listener, conn_pool)?.await
}
