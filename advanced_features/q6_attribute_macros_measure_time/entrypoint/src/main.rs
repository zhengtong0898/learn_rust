use measure_time::measure_time;
use std::thread;

/*
Attribute-like macros / 属性宏

可以当作装饰器来使用, 为每个挂载了#[measure_time]宏的函数
都增加一段计时片段代码, 用于度量函数的执行时间.
*/

#[measure_time]
fn do_something() -> bool {
    thread::sleep(std::time::Duration::from_secs(2));
    false
}

fn main() {
    let _ = do_something();
}
