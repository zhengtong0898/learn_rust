use std::thread;
use tokio::runtime::Builder;
use tokio::time::{sleep, Duration};

fn main() {
    // 使用 Builder 来设置运行时
    // 如果设置成1个线程, 那么下面的task0就会堵塞所有后续的代码, 因为它内部使用了一个堵塞的sleep, 所有的异步任务都需要先等堵塞的代码执行完成之后才能执行.
    // 如果设置成2个线程, 那么限免的task0就只堵塞一个线程, 后续所有的task都会同时的在另外一个线程中执行.
    let rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let task0 = tokio::spawn(async_task_with_block_sleep(
            "任务0",
            Duration::from_secs(10),
        ));
        let task1 = tokio::spawn(async_task("任务1", Duration::from_secs(3)));
        let task2 = tokio::spawn(async_task("任务2", Duration::from_secs(2)));
        let task3 = tokio::spawn(async_task("任务3", Duration::from_secs(1)));

        // 等待所有任务完成
        let results = tokio::try_join!(task0, task1, task2, task3);

        // 检查结果
        match results {
            Ok(_) => println!("所有任务成功完成"),
            Err(e) => println!("任务出错: {:?}", e),
        }
    });
}

async fn async_task(name: &str, duration: Duration) -> String {
    println!("{} 开始执行，将等待 {:?} 秒...", name, duration.as_secs());
    sleep(duration).await;
    let result = format!("{} 完成", name);
    println!("{}", result);
    result
}

async fn async_task_with_block_sleep(name: &str, duration: Duration) -> String {
    println!("{} 开始执行，将等待 {:?} 秒...", name, duration.as_secs());
    thread::sleep(duration);
    let result = format!("{} 完成", name);
    println!("{}", result);
    result
}
