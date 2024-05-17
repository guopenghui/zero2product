pub mod configurations;
pub mod routes;
pub mod startup;

use actix_web::dev::Server;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[get("/health_check")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/subscriptions")]
async fn subscribe(data: web::Form<FormData>) -> impl Responder {
    let _ = data.name;
    let _ = data.email;
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(greet).service(subscribe))
        .listen(listener)?
        .run();

    Ok(server)
}
