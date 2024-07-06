use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

use carbonite::{
    configuration::{DatabaseSettings, Settings},
    startup::{get_connection_pool, Application},
    telemetry::{get_subscriber, init_subscriber},
};

use once_cell::sync::Lazy;

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), "debug".into(), std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), "debug".into(), std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct ConfirmationLinks {
    pub html: String,
    pub plain_text: String,
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
    pub port: u16,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub fn get_confirmation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

        let get_links = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();
            assert_eq!(links.len(), 1);
            links[0].as_str().to_owned()
        };

        let html = get_links(&body["HtmlBody"].as_str().unwrap());
        let plain_text = get_links(&body["TextBody"].as_str().unwrap());
        ConfirmationLinks { html, plain_text }
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let email_server = MockServer::start().await;
    let configuration = {
        let mut c = Settings::from_file().expect("failed to read configuration");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c.email_client.base_url = email_server.uri();
        c
    };
    configure_database(&configuration.database).await;
    let application = Application::build(configuration.clone())
        .await
        .expect("failed to build application");

    let application_port = application.port();
    let address = format!("http://127.0.0.1:{}", application_port);

    let _ = actix_web::rt::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
        email_server,
        port: application_port,
    }
}

async fn configure_database(db_setting: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect_with(&db_setting.without_db())
        .await
        .expect("Failed to connect to Postgres.");
    conn.execute(format!(r#"CREATE DATABASE "{}";"#, db_setting.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let conn_pool = PgPool::connect_with(db_setting.with_db())
        .await
        .expect("failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to migrate the database");

    conn_pool
}
