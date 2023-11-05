use log::{error, info, warn};
use std::io::Write;

fn main() {
    println!("Current RUST_LOG: {:?}", std::env::var("RUST_LOG"));

    // 从环境变量中读取RUST_LOG
    let mut log_builder = env_logger::Builder::from_env(env_logger::Env::default());

    // 2. 设置时间格式为纳秒
    log_builder.format(|buf, record| {
        let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.9f");
        writeln!(buf, "{} [{}] - {}", ts, record.level(), record.args())
    });

    // 3. 初始化日志记录器
    log_builder.init();

    // 记录不同级别的日志
    info!("This is an info message");
    warn!("This is a warning");
    error!("This is an error");
}
