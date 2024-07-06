use carbonite::configuration::Settings;
use carbonite::startup::Application;
use carbonite::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("carbonite".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = Settings::from_file().expect("failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
