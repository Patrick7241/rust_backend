use actix_cors::Cors;

// TODO 开发环境允许所有来源，生产环境需要修改配置
pub fn config_cors() -> Cors {
    Cors::permissive()
}
