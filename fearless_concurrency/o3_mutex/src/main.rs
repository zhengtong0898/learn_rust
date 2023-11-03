use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc -> Atomic Rc: 使对象支持原子性引用
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // 原子引用计数+1, 克隆一份所有权出来.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        // 将所有thread_handle句柄保存到handles这个vec!中.
        handles.push(handle);
    }

    // 遍历所有thread句柄, 挨个join等待其结束.
    for handle in handles {
        handle.join().unwrap();
    }

    // 注意:
    // *counter.lock().unwrap() 的执行顺序是:
    // let ss1: Result<std::sync::MutexGuard<'_, i32>, std::sync::PoisonError<std::sync::MutexGuard<'_, i32>>> = counter.lock();
    // let ss2: std::sync::MutexGuard<'_, i32> = ss1.unwrap();
    // let ss3: i32 = *ss2;
    println!("Result: {}", *counter.lock().unwrap());
}
