use std::io;

use tracing_appender::non_blocking;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter, Registry};
use tracing_subscriber::prelude::*;
use time::macros::format_description;
use tracing_subscriber::fmt::time::LocalTime;

pub fn init_logging() -> (io::Result<()>, tracing_appender::non_blocking::WorkerGuard) {
    // 设置日志过滤器，默认 DEBUG，可用 RUST_LOG 覆盖
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("rust_blog=info"));  // 过滤掉非本crate的日志内容

    // 滚动日志配置
    let rolling_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    let (non_blocking_appender, guard) = non_blocking(rolling_appender);


    let timer = LocalTime::new(format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"));

    let console_layer = fmt::layer()
        .with_writer(io::stdout)
        .with_ansi(true)
        .with_thread_ids(false)
        .with_timer(timer.clone())
        .with_target(false);

    let file_layer = fmt::layer()
        .with_writer(non_blocking_appender)
        .with_ansi(false)
        .with_thread_ids(false)
        .with_timer(timer)
        .with_target(false);


    // 注册 subscriber
    let subscriber = Registry::default()
        .with(filter)
        .with(console_layer)
        .with(file_layer);

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to set tracing subscriber: {}", e)))
        .map(|_| ()).expect("init logger error");

    (Ok(()), guard)
}
