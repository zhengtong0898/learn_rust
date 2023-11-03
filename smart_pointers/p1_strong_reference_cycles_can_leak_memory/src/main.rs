use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

/*
强类型的循环引用会导致
1). print、json时出现内存泄漏.
2). 循环引用的变量的引用计数无法消减为0, 从而在程序运行的期间永远驻留在内存中无法被释放.
*/

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // 创建a变量
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // a变量是 List 枚举的Cons成员类型, 所以可以使用 List 的方法.
    // 此时 a 变量的引用计数是: 1
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    println!("----------------------------");

    // 创建b变量, 并在Cons的第二个类型上引用a变量, 简称b指向a
    // 此时 a 变量的引用计数是: 2;
    // 此时 b 变量的引用计数是: 1;
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    println!("----------------------------");

    // 变量a指向b
    // 此时a和b形成了一个环形引用.
    // 此时 a 变量的引用计数是: 2;
    // 此时 b 变量的引用计数是: 2;
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    println!("a next item = {:?}", a.tail());
}
