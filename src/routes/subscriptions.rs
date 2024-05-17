use actix_web::{post, web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscribe(data: web::Form<FormData>) -> impl Responder {
    let _ = data.name;
    let _ = data.email;
    HttpResponse::Ok()
}
