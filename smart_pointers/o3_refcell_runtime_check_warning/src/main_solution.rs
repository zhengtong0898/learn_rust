use std::cell::RefCell;

#[derive(Debug)]
struct Foo {
    id: i32,
}

fn main() {
    let foo = RefCell::new(Foo { id: 1 });

    // 手动模拟编译器在编译期的Non-Lexical-Lifetimes行为, 即:
    // loan在这两行代码结束后就不在使用了, 给他们增加一个作用域, 让它失效.
    {
        let loan = foo.borrow();
        println!("{:?}", loan.id);
    }

    let mut other = foo.borrow_mut();
    other.id = 15;
    println!("{:?}", other.id);
}
