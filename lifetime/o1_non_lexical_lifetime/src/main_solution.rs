#[derive(Debug)]
struct Foo {
    id: i32,
}

impl Foo {
    // 根据lifetime Elision的第三条原则说明: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
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
    // 尽管返回的是一个`&Self`，但由于它是从 `&mut self` 派生的，所以rust会认为 `&Self` 即是不可变引用, 也是可变引用.
    // 这就会导致在`&Self`被回收之前, 都不可以被不可变引用赋值给其他变量.
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}

fn main() {
    let mut foo = Foo { id: 1 }; // foo 是一个可变变量, 此时它的不可变引用是0, 它的可变引用是0.

    foo.id = 15; // foo 是一个可变变量, 所以可以修改它的值.

    let loan = foo.mutate_and_share(); // 此时foo的不可变引用是1, 它的可变引用是1.

    println!("{:?}", loan); // 后续代码都不再使用loan, rust编译器在这里回收loan,
                            // 此时foo的不可变引用是0, 它的可变引用是0.
                            // 这是2018年的时候Rust 1.31.0推出的Non-Lexical Lifetimes特性,
                            // 专门处理这种情况: 当后续代码不在使用loan了, 那么就允许释放loan的foo引用.
                            // 但是需要注意的是loan变量依旧存在, 知道main函数结束时才会被回收.

    foo.share(); // 不可变引用成功, 说明loan的生命周期已结束.
    foo.id = 20; // foo变量可以正常使用, 说明loan的生命周期已结束.
}
