use std::fmt;

// Refer: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-supertraits-to-require-one-traits-functionality-within-another-trait
//
// `trait` 的 `super` 指的是:
// 1. `OutlinePrint` 依赖 `fmt::Display` 这个 `trait`.
// 2. `OutlinePrint.outline_print` 方法内可以使用 `fmt::Display`` 中定义的方法 `self.to_string()`, 这就相当于是继承.

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
struct Point {
    x: i32,
    y: i32,
}

// 如果impl块内没做任何实现, 那么就会自动继承OutlinePrint的默认实现的方法.
impl OutlinePrint for Point {}

// 为 Point 实现 fmt::Display.fmt 接口方法.
// 这个方法被outline_print方法中的 self.to_string() 触发.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1, y: 3 };
    p.outline_print();
}
