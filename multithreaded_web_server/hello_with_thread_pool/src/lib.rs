use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// Box    是一个指针, 他拥有固定大小, 它支持Deref(解引用)、它支持Drop(离开作用域时自动执行drop方法).
// Box<T> 其中的T就是一个类型, 这里这一戳东西可以被看作是一个类型, 这个类型被这几个trait约束:
//        1). dyn FnOnce():  一个闭包(只能执行一次), 它默认会move所有捕获的变量.
//        2). Send:          这个闭包必须是线程安全的, 也就是这个闭包内不可以捕获Rc<T>对象.
//        3). 'static:       这个闭包内所捕获的对象必须都是'static生命周期的对象.
//                           以main.rs中的 pool.execute(||{ handle_connection(stream); }) 来举例说明.
//                           在这个闭包中捕获了stream对象，因为采取的是move所有权转移, 所以stream变成了闭包
//                           的一部分, 因此 stream 在闭包内也会被视为满足'static.
//
// 最后需要注意的是:
// 如果闭包只是 Send 而不是 'static，则在线程之间传递闭包可能是不安全的，因为原始引用可能已经失效。
//
// 总的来说, 这行代码的意思是:
// Job 是一个Box智能指针, 指向的类型是 一个线程安全的、采取move策略(转移所有权)的闭包.
type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(unused_variables)]
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // 为了能在多线程中传递, 这里采用Arc来做原子性的引用计数.
        let receiver = Arc::new(Mutex::new(receiver));

        // 创建指定数量的Worker, 保存到Vector中, Vector变量名是workers.
        // 每个worker都是一个消费者, 这意味着每个worker都是一个for无限循环接收数据.
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // 返回Threadpool结构体, 该结构体包含了已创建的workders和sender.
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

#[allow(unused_variables)]
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // receiver 是一个智能指针, 因此可以直接操作指针, 通过 receiver.lock() 可以直接对Mutex进行操作.
                // let local_result = receiver.lock();
                // let mutex_guard = local_result.unwrap();
                // let result_fnonce = mutex_guard.recv();
                // let fnonce = result_fnonce.unwrap();
                // let job = receiver.lock().unwrap().recv().unwrap();

                let result = receiver.lock().unwrap().recv();
                match result {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
