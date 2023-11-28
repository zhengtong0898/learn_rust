use std::sync::Arc;
use std::time::Instant;
use tokio;
use tokio::sync::RwLock;

struct Person {
    name: Arc<RwLock<String>>,
}

#[tokio::main]
async fn main() {
    let person = Person {
        name: Arc::new(RwLock::new("zhangsan".to_string())),
    };

    // 参考: 没有读写锁、没有异步等待, 直接clone是 100 nanos;

    let start_time = Instant::now();
    let name = person.name.read().await.clone(); // debug: 5600 nanos; release: 900 nanos;
    let duration = Instant::now().duration_since(start_time);

    println!(
        "Hello, world! name: {}; elapsed: {};",
        name,
        duration.as_nanos()
    );
}
