use actix_web::{web,middleware::from_fn};
use crate::handlers::user::login;
use crate::middleware::authorization::config_authorization;

pub fn config(cfg: &mut web::ServiceConfig) {
    let user_scope = web::scope("/user")
        .wrap(from_fn(config_authorization))
        .service(login);

    cfg.service(user_scope);
}