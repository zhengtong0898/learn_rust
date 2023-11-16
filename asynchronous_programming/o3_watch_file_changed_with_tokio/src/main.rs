use anyhow::{self, Ok};
use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 启动一个新的异步task来运行这个代码, 确保不堵塞当前的异步任务.
    tokio::spawn(async {
        watch().await.unwrap();
    });

    Ok(())
}

async fn watch() -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel(10);

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            tx.blocking_send(res).unwrap();
        },
        NotifyConfig::default(),
    )?;

    // 添加要监控的路径
    watcher.watch(Path::new("main.log"), RecursiveMode::Recursive)?;

    while let Some(_) = rx.recv().await {
        // 发送事件到异步通道
        println!("Receive notify")
    }

    Ok(())
}
