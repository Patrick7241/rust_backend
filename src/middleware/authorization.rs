use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error, HttpMessage, HttpResponse,
    http::StatusCode,
};
use actix_web::error::ErrorUnauthorized;
use crate::common::http::ApiResponse;

// 定义一个结构体来存储解析后的用户信息
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub user_id: String,
    pub username: String,
}

pub async fn config_authorization(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let auth_header = req.headers().get("Authorization");

    match auth_header {
        Some(header_value) => {
            let _auth_str = header_value
                .to_str()
                .map_err(|_| ErrorUnauthorized("Invalid authorization header"))?;

            println!("{}", _auth_str);
            // TODO: 校验 token

            next.call(req).await
        }
        None => {
            let resp = HttpResponse::Unauthorized()
                .json(ApiResponse::<()>::error(401, "No authorization header provided"));
            Ok(req.into_response(resp))
        }
    }
}