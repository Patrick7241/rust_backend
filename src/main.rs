use actix_web::{App, HttpServer};
use tokio::{signal,select};
use tracing::{info};

mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let (_res, _guard) = middleware::log::init_logging();
    
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::cors::config_cors())
            .service(actix_web::web::resource("/").to(|| async { "Hello World!" }))
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