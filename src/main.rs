use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use tokio::{signal, select};
use tracing::{info};
use anyhow::Result;

mod middleware;
mod routes;
mod handlers;
mod common;
mod services;
mod db;

#[actix_web::main]
async fn main() -> Result<()> {
    // 日志初始化
    let (_res, _guard) = middleware::log::init_logging();

    // 数据库初始化
    db::mysql::init_db();

    let server = HttpServer::new(|| {
        App::new()
            // 添加响应日志
            .wrap(Logger::default())
            // 跨域配置
            .wrap(middleware::cors::config_cors())
            // 挂载路由
            .configure(routes::init_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run();

    info!("🚀 Server running at http://127.0.0.1:8080");

    // 获取服务器句柄用于优雅关闭
    let server_handle = server.handle();

    select! {
        _ = server => {
            info!("服务器自然停止");
        }
        _ = async {
            signal::ctrl_c()
                .await
                .expect("无法监听 Ctrl+C");
            info!("🛑 收到 Ctrl+C，正在优雅关闭...");
        } => {
            server_handle.stop(true).await;
        }
    }

    Ok(())
}