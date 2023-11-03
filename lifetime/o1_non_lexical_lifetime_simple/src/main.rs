#[derive(Debug)]
struct Foo {
    id: i32,
}

fn main() {
    let mut foo = Foo { id: 1 };

    // Rust 于 2018年推出的Rust 1.31.0版本, 开始支持Non-Lexical-Lifetimes特性, 即:
    // 在编译期做借用规则检查时, 如果发现一个变量在后续的代码中都不在使用时就会消除掉它的引用, 让后续的其他代码可以继续引用.
    let loan = &foo;
    println!("{:?}", loan.id);
    // 可以这样理解, 它将代码做了一些改变, 给这个代码片段增加了一个作用域, 让它在离开作用域时被回收.
    // {
    //     let loan = &foo;
    //     println!("{:?}", loan.id);
    // }

    let other = &mut foo;
    println!("{:?}", other.id);
}
