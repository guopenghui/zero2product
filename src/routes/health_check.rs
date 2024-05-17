use actix_web::{get, HttpResponse, Responder};

#[get("/health_check")]
pub async fn greet() -> impl Responder {
    HttpResponse::Ok().finish()
}
