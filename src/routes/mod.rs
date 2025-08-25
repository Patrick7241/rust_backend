pub mod ping;
mod user;

use actix_web::web;

type RouteConfig = fn(&mut web::ServiceConfig);

// 一键注册所有路由
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // 所有路由模块的配置函数集合
    let routes: &[RouteConfig] = &[
        ping::config,
    ];
    let api_routes: &[RouteConfig] = &[
        user::config,
    ];

    // 注册
    for r in routes {
        r(cfg);
    }

    let mut api_scope = web::scope("/api");
    for r in api_routes {
        api_scope = api_scope.configure(*r);
    }
    cfg.service(api_scope);
}
