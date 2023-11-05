use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(async_task("任务1", Duration::from_secs(3)));
    let task2 = tokio::spawn(async_task("任务2", Duration::from_secs(2)));
    let task3 = tokio::spawn(async_task("任务3", Duration::from_secs(1)));

    // 等待所有任务完成
    let results = tokio::try_join!(task1, task2, task3);

    // 检查结果
    match results {
        Ok(_) => println!("所有任务成功完成"),
        Err(e) => println!("任务出错: {:?}", e),
    }
}

async fn async_task(name: &str, duration: Duration) -> String {
    println!("{} 开始执行，将等待 {:?} 秒...", name, duration.as_secs());
    sleep(duration).await;
    let result = format!("{} 完成", name);
    println!("{}", result);
    result
}
