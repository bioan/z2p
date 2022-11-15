mod configuration;
use configuration::get_configuration;
use std::net::TcpListener;

#[allow(unused_must_use)]
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application_host, configuration.application_port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind to random port");
    println!("Running on {}", listener.local_addr().unwrap().port());
    let app = zero2prod::startup::run(listener);
    app?.await;
    Ok(())
}
