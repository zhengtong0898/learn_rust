// 声明式宏 Declarative Macros
// Macros 详细的语法: https://doc.rust-lang.org/reference/macros-by-example.html

// #[macro_export] 表示这是一个对外开放的宏, 外部crate可以导入并使用.
#[macro_export]
// macro_rules! 表示这是一个声明式宏.
macro_rules! print_hello {
    // () => {} 表示当调用print_hello!这个宏的时候, 如果没有提供任何参数, 那么就会执行这个代码块.
    () => {
        println!("Hello from my_crate!");
    };

    // expr          是一个表达式
    // ele           是一个变量名
    // ,             重复的表达式采用逗号来分隔, 允许的可定义分隔符是: "=>" 、 "," 、";"
    // +             接收至少一个参数
    //
    // $()           表示开始一个匹配
    // $($x:expr)    表示把表达式的结果赋值给$x这个宏变量
    // $($x:expr),+  表示重复多次匹配, 每次匹配以逗号作为重复匹配的依据
    //               例如: print_hello!("nihao", 31+5, false); 就被匹配了三次.
    // $(,)?         在$($x:expr),+的右侧继续匹配1个或0个逗号.
    //
    // $(ss+=...)+   每次匹配到$x的时候都会执行一次主体中的$()中的代码.
    ($($x:expr),+$(,)?) => {
        let mut ss = String::from("");
        $(
            ss += &format!(" {:?} ", $x);
        )+
        println!("{}", ss);
    };
}

fn main() {
    // 调用print_hello!宏.
    print_hello!("nihao", 31 + 5, false);
}
