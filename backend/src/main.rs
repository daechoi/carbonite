use carbonite::configuration::Settings;
use carbonite::email_client::EmailClient;
use carbonite::startup::run;
use carbonite::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("carbonite".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = Settings::from_file().expect("failed to read configuration");
    let sender_email = settings
        .email_client
        .sender()
        .expect("invalid sender email address");

    let email_client = EmailClient::new(
        sender_email,
        settings.email_client.base_url,
        settings.email_client.authorization_token,
    );

    let conn_pool = PgPoolOptions::new()
        .idle_timeout(Duration::from_secs(2))
        .connect_lazy_with(settings.database.with_db());

    let listener = TcpListener::bind(format!(
        "{}:{}",
        settings.application.host, settings.application.port
    ))
    .expect("Failed to bind to 8080");
    run(listener, conn_pool, email_client)?.await
}
