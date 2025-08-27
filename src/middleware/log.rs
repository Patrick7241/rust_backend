use std::io;

use tracing_appender::non_blocking;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter, Registry};
use tracing_subscriber::prelude::*;
use time::macros::format_description;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_log::LogTracer;

pub fn init_logging() -> (io::Result<()>, non_blocking::WorkerGuard) {
    // 使 log crate 的日志转发给 tracing
    LogTracer::init().expect("Failed to set LogTracer");

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("rust_backend=info,actix_web=info"));

    let rolling_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    let (non_blocking_appender, guard) = non_blocking(rolling_appender);

    let timer = LocalTime::new(
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]")
    );

    let console_layer = fmt::layer()
        .with_writer(io::stdout)
        .with_ansi(true)
        .with_timer(timer.clone());

    let file_layer = fmt::layer()
        .with_writer(non_blocking_appender)
        .with_ansi(false)
        .with_timer(timer);

    let subscriber = Registry::default()
        .with(filter)
        .with(console_layer)
        .with(file_layer);

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to set tracing subscriber: {}", e)))
        .map(|_| ()).expect("init logger error");

    (Ok(()), guard)
}