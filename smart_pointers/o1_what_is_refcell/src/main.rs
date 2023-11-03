use std::cell::{Ref, RefCell, RefMut};

#[allow(unused_variables)]
fn main() {
    // Rust 在编译期不检查 RefCell 对象的借用规则, 让程序在运行时检查.
    //
    // 最终表现是:
    // 1). 编译期即便有问题也不会报错, 即编译成二进制程序没问题.
    // 2). 运行时如果有问题就直接panic, 然后推出程序.
    let x: RefCell<Vec<f64>> = RefCell::new(vec![1.0, 2.0, 3.0]);
    let y: RefMut<'_, Vec<f64>> = x.borrow_mut();
    let z: Ref<'_, Vec<f64>> = x.borrow(); // This causes a panic
}
