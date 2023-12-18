use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;

use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();

    Ok(server)
}