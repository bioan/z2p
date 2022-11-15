use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, App, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}
