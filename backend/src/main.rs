use std::net::TcpListener;

use carbonite::configuration::Settings;
use carbonite::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::from_file("config/config.yaml").expect("failed to read configuration");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.application_port))
        .expect("Failed to bind to 8080");
    run(listener)?.await
}
