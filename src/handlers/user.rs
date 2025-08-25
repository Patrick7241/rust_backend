use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::common::http::ApiResponse;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}
#[derive(serde::Serialize)]
struct LoginResponse {
    token: String,
}

#[post("/login")]
pub async fn login(payload: Result<web::Json<LoginRequest>, actix_web::Error>) -> impl Responder {
    match payload {
        Ok(data) => {
            match crate::services::user::login(&data.username, &data.password) {
                Ok(token) => {
                    HttpResponse::Ok().json(ApiResponse::success(LoginResponse { token }))
                }
                Err(e) => {
                    HttpResponse::Unauthorized().json(ApiResponse::<()>::error(401, e.to_string()))
                }
            }
        }
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(400, e.to_string())),
    }
}