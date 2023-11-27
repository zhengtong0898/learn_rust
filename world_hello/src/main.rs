use std::time::Instant;

#[allow(unused_variables)]
fn main() {
    let start_time = Instant::now(); // 耗时: < 100 nanos
    let x = 5; //                           耗时: < 100 nanos
    let s = 6; //                           耗时: < 100 nanos
    let ss = "good"; //                    耗时: < 100 nanos
    let sss = "hello world!".to_string(); //                                               耗时: debug:    500 nanos; release:    200 nanos;
    let duration = Instant::now().duration_since(start_time); // 前面5行代码加起来耗时: debug:    700 nanos; release:    100 nanos;
    println!("assign variable: {}", duration.as_nanos()); //                                这行代码耗时: debug: 110,000 nano; release: 53,000 nanos;

    let duration = Instant::now().duration_since(start_time); //          这行代码耗时: debug:    100 nanos; release:    100 nanos;
    println!("assign variable: {}", duration.as_nanos());
}
