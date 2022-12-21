use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let respone = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(respone.status().is_success());
    assert_eq!(Some(0), respone.content_length());
}

#[tokio::test]
async fn add_suscriber_is_success() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=forevalone&email=forevalone%40gmail.com";
    let config = get_configuration().expect("Failed to read configuration");
    let connection_string = config.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    let saved = sqlx::query!("Select email, name from subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch suscription");
    let respone = client
        .post(&format!("{}/suscriptions", &addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");
    assert!(respone.status().is_success());
    assert_eq!(saved.email, "forevalone%40gmail.com");
    assert_eq!(saved.name, "forevalone");
}

#[tokio::test]
async fn add_suscriber_with_invalid_body_fails() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let body = vec![
        ("email=forevalone%40gmail.com", "Name missing"),
        ("name=forevalone", "Email missing"),
        ("", "Body missing"),
    ];
    for (invalid_body, err_description) in body {
        let respone = client
            .post(&format!("{}/suscriptions", &addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");
        assert_eq!(
            400,
            respone.status().as_u16(),
            "Request didnt fail for payload = {}",
            err_description
        );
    }
}
