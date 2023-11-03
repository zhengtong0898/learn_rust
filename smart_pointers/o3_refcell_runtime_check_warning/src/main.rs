use std::cell::RefCell;

/*
下面这段代码有几个特征:
1. 编译可以通过, 因为使用了RefCell会绕过编译期的借用规则检查.
2. 运行会失败, 因为在main这个作用域中, loan这个不可变引用会一直生效到main作用域结束,
   因此后续的other可变引用是不成立的, 故此程序崩溃(报panic).
3. 运行时做借用规则检查, 并不会采取Non-Lexical-Lifetimes规则, 参考: ../lifetime/o1_non_lexical_lifetime_simple
*/

#[derive(Debug)]
struct Foo {
    id: i32,
}

fn main() {
    let foo = RefCell::new(Foo { id: 1 });

    let loan = foo.borrow();
    println!("{:?}", loan.id);

    let mut other = foo.borrow_mut();
    other.id = 15;
    println!("{:?}", other.id);
}
