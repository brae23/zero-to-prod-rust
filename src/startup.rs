use actix_web::dev::Server;
use actix_web::{web::{self}, App, HttpServer};
use std::net::TcpListener;
use crate::routes::{subscribe, health_check};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();
    Ok(server)
}