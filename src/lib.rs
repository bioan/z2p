use std::net::TcpListener;

use actix_web::{web, dev::Server, get, post, App, HttpResponse, HttpServer, Responder};

#[derive(serde::Deserialize)]
struct SubscriptionData {
    name: String,
    email: String
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<SubscriptionData>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}
