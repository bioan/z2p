use std::net::TcpListener;

#[allow(unused_must_use)]
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to random port");
    println!("Running on {}", listener.local_addr().unwrap().port());
    let app = zero2prod::startup::run(listener);
    app?.await;
    Ok(())
}
