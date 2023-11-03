#[derive(Debug)]
struct Foo {
    id: i32,
}

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}

#[allow(unused_variables)]
fn main() {
    // foo 是一个可变变量.
    let mut foo = Foo { id: 10 };

    // foo 是一个可变变量, 所以可以修改它的值.
    foo.id = 15;

    // 从 mutate_and_share 函数中看它的返回值是&Self, 即不可变引用.
    // 但是根据lifetime Elision的第三条原则说明: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
    // The third rule is that, if there are multiple input lifetime parameters,
    // but one of them is &self or &mut self because this is a method,
    // the lifetime of self is assigned to all output lifetime parameters.
    // This third rule makes methods much nicer to read and write because fewer symbols are necessary.
    //
    // 所有的返回值的生命周期和self一样长, 那么mutate_and_share函数长成这个样子:
    // fn mutate_and_share<'a>(&'a mut self) -> &'a Self {
    //     &'a *self
    // }
    //
    // 注意:
    // 1). loan 是一个不可变引用的变量, 所以后续的loan.id = 20是不合法的代码.
    // 2). loan 是从 `&mut foo` 中派生出来的 `&foo`, Rust为了确保内存安全, Rust将禁止`foo`再次被借用.
    //     这就好比 loan 是一把 `foo` 的读写锁, 它首先是通过写锁把`foo`锁住, 然后又把读锁给返回给`loan`,
    //     这就造成了 `foo` 被写锁住了, 同时也被读锁住了.
    //     这种情况只有当 loan 变量失效后, `foo`才能再次被使用, 否则Rust将会再编译器直接报错,
    //     为了解决这种问题, Rust 1.31.0推出的Non-Lexical Lifetimes特性,
    //     当后续代码不在使用loan了, 那么就允许释放loan的foo引用,
    //     完整的解决方案请参考: main_solution.rs 源码.
    let loan = foo.mutate_and_share();

    // 由于loan是不可变引用, 所以这里是不可以修改loan的值的.
    loan.id = 20;

    // 由于 foo 是被 loan 可变引用状态, 此时foo是不可用状态, 必须等loan释放.
    foo.share();

    println!("{:?}", loan);
}
