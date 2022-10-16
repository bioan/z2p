//! tests/health_check.rs

#[tokio::test]
#[allow(clippy::unused_must_use)]
async fn health_check_test(){
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://0.0.0.0:8080/health_check")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

use tokio;

async fn spawn_app(){
    let server = zero2prod::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}