use tracing::{info, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{self, RollingFileAppender};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter, fmt::layer, fmt::time::ChronoLocal, Registry};

struct GuardHolder(Vec<WorkerGuard>);

fn init_logger() -> GuardHolder {
    let main_appender = RollingFileAppender::new(rolling::Rotation::DAILY, "logs/", "main.log");
    let (main_non_blocking, main_guard) = tracing_appender::non_blocking(main_appender);
    let main_layer = layer()
        .with_ansi(false)
        .with_writer(main_non_blocking)
        .with_timer(ChronoLocal::with_format("%Y-%m-%d %H:%M:%S.%f".to_string()))
        .with_filter(filter::Targets::new().with_target("main", Level::INFO));

    let console_appender =
        RollingFileAppender::new(rolling::Rotation::DAILY, "logs/", "console.log");
    let (console_non_blocking, console_guard) = tracing_appender::non_blocking(console_appender);
    let console_layer = layer()
        .with_ansi(false)
        .with_writer(console_non_blocking)
        .with_timer(ChronoLocal::with_format("%Y-%m-%d %H:%M:%S.%f".to_string()))
        .with_filter(filter::Targets::new().with_target("console", Level::INFO));

    Registry::default()
        .with(main_layer)
        .with(console_layer)
        .init();

    GuardHolder(vec![main_guard, console_guard])
}

fn main() {
    // 定义了两个日志文件: main.log 和 console.log
    let _guard_holder = init_logger();

    // 发送日志消息: 写入到 main.log
    info!(target: "main", "This log message is for business A");

    // 发送日志消息: 写入到 console.log
    info!(target: "console", "This log message is for business B");
}
