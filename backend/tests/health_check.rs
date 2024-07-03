use carbonite::configuration::Settings;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    let addr = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let settings =
        Settings::from_file("config/config.yaml").expect("Failed to read configuration.");

    let connstr = settings.database.connection_string();
    let conn = PgPool::connect(&connstr)
        .await
        .expect("Failed to connect to Postgres.");

    let server = carbonite::startup::run(listener, conn).expect("Failed to bind address");

    let _ = actix_web::rt::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[actix_web::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app().await;
    let settings = Settings::from_file("config/config.yaml").unwrap();
    let connstr = settings.database.connection_string();
    let conn = PgPool::connect(&connstr)
        .await
        .expect("Failed to connect to Postgres.");

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",) // 1
        .fetch_one(&conn) // 2
        .await // 3
        .expect("Failed to fetch saved subscription."); // 4

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_web::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
