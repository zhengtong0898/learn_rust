/*
Refer: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

The advantage of checking the borrowing rules at runtime instead is that certain memory-safe
scenarios are then allowed, where they would’ve been disallowed by the compile-time checks. Static
analysis, like the Rust compiler, is inherently conservative. Some properties of code are impossible
to detect by analyzing the code: the most famous example is the Halting Problem, which is beyond
the scope of this book but is an interesting topic to research.

有些场景是内存安全的, 但是Rust在编译期做借用规则检查时缺报不符合内存安全原则.
这种情况, 使用 RefCell 就能让代码放弃编译期检查, 转而要求程序在运行时在做检查.
这种模式是一种权衡后的折中做法, 这为开发者提供了更大的灵活性, 但代价是在运行时可能会出现崩溃、性能开销.

没有案例说明, 以后再补充吧.
*/

use std::collections::HashMap;

struct CachedCalculator {
    cache: HashMap<i32, i32>,
}

impl CachedCalculator {
    fn new() -> Self {
        CachedCalculator {
            cache: HashMap::new(),
        }
    }

    // 2. self本身是不可变的, 因此不能修改内部的值.
    fn calculate(&self, value: i32) -> i32 {
        // 首先检查缓存
        if let Some(&cached_result) = self.cache.get(&value) {
            return cached_result;
        }

        // 计算（这里简化为 value * value）
        let result = value * value;

        // 尝试存入缓存
        self.cache.insert(value, result); // 这里会导致编译错误

        result
    }
}

fn main() {
    // 1. 不可变的变量.
    let calculator = CachedCalculator::new();
    assert_eq!(calculator.calculate(2), 4);
    assert_eq!(calculator.calculate(3), 9);
    assert_eq!(calculator.calculate(2), 4);
}

// use std::cell::RefCell;

// fn main() {
//     let data = RefCell::new(String::from("Hello, RefCell!"));

//     if get_runtime_flag() {
//         let borrowed = data.borrow();
//         print_data(&borrowed);
//         // 这里的 borrowed 作用域结束, 所以不再借用 data

//         // 现在我们可以尝试可变地借用 data
//         let mut mut_borrow = data.borrow_mut();
//         mut_borrow.push_str(" Some more data.");
//         println!("{}", mut_borrow);
//     } else {
//         let mut mut_borrow = data.borrow_mut();
//         mut_borrow.push_str(" Some other data.");
//         println!("{}", mut_borrow);
//     }
// }

// fn get_runtime_flag() -> bool {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();

//     input.trim() == "use"
// }

// fn print_data(data: &String) {
//     println!("{}", data);
// }
