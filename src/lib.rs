pub mod configurations;
pub mod routes;
pub mod startup;

use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;

use routes::{greet, subscribe};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(greet).service(subscribe))
        .listen(listener)?
        .run();

    Ok(server)
}
