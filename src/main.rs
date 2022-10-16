use std::net::TcpListener;

use zero2prod::run;
#[allow(unused_must_use)]
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to random port");
    let app = run(listener);
    app?.await;
    Ok(())
}
