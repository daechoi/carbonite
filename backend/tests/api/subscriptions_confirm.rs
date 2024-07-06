use reqwest::Url;
use wiremock::matchers::{method, path};
use wiremock::Mock;
use wiremock::ResponseTemplate;

use crate::helpers::spawn_app;

#[actix_web::test]
async fn confirmation_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}

#[actix_web::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];

    let confirmation_links = app.get_confirmation_links(email_request);
    let mut confirmation_link = Url::parse(confirmation_links.html.as_str()).unwrap();
    assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
    confirmation_link.set_port(Some(app.port)).unwrap();

    let response = reqwest::get(confirmation_link).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);
}
