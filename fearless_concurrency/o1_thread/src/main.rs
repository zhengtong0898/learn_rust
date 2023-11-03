use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    // 通过使用move关键字, 告诉Rust, 闭包应该拥有这些值的所有权, 而不仅仅借用他们的引用.
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!; {:?}", i, v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 由于v的所有权已经转移到线程中, 所以这里不能再继续使用v对象.
    // println!("{:?}", v);

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待线程执行完毕.
    handle.join().unwrap();
}
