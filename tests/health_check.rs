//! tests/health_check.rs

#[tokio::test]
// #[allow(unused_must_use)]
async fn health_check_test(){
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

use std::net::TcpListener;

use tokio;

fn spawn_app() -> String{
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    format!("http://0.0.0.0:{}", port)
}