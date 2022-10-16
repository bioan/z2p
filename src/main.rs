use zero2prod::run;
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let x = run();
    x?.await
}