use actix_web::{post, HttpResponse, Responder};

#[post("/login")]
pub async fn login() -> impl Responder{
    HttpResponse::Ok().body("pongsdf")
}