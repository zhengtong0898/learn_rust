struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // What is Associated Type in Trait?
    //
    // Refer: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
    //
    // Specifying Placeholder Types in Trait Definitions with Associated Types
    // 在定义 `Trait` 的时候, 使用 `Associated Type` 来占位.
    //
    //   pub trait Iterator {
    //       type Item;
    //
    //       fn next(&mut self) -> Option<Self::Item>;
    //   }
    //
    // 从整体的角度上看, `type Item` 是一个 `Placeholder Type`,
    // 从方法的角度上看, `Self::Item` 是一个 `Associated Type`, 这两个术语是相同的含义.
    //
    //
    // Associated types connect a type placeholder with a trait such that
    // the trait method definitions can use these placeholder types in their signatures.
    // 有了 `Associated types`, 就可以在函数形参、返回值的地方使用这个`Associated types`,
    // 用 `Associated types` 来替代具体类型.

    // 隐式循环遍历
    let counter = Counter::new();
    for num in counter {
        println!("{}", num);
    }

    println!("\n--------------------------------------\n");

    // 显式循环遍历
    let other_counter = Counter::new();
    let mut other_counter_iter = other_counter.into_iter();
    println!("{}", other_counter_iter.next().unwrap());
    println!("{}", other_counter_iter.next().unwrap());
    println!("{}", other_counter_iter.next().unwrap());
    println!("{}", other_counter_iter.next().unwrap());
    println!("{}", other_counter_iter.next().unwrap());
    println!("{}", other_counter_iter.next().unwrap());
}
