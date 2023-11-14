use std::cell::RefCell;
use std::rc::Rc;

// 定义结构体 A
#[derive(Debug)]
struct A {
    id: String,
    name: String,
    binded_opposite_odds: RefCell<Option<Rc<B>>>,
}

// 定义结构体 B
#[derive(Debug)]
struct B {
    id: String,
    name: String,
    binded_opposite_odds: RefCell<Option<Rc<A>>>,
}

impl A {
    fn new(id: &str, name: &str) -> Rc<Self> {
        Rc::new(Self {
            id: id.to_string(),
            name: name.to_string(),
            binded_opposite_odds: RefCell::new(None),
        })
    }

    fn bind(&self, b: &Rc<B>) {
        *self.binded_opposite_odds.borrow_mut() = Some(Rc::clone(b));
    }
}

impl B {
    fn new(id: &str, name: &str) -> Rc<Self> {
        Rc::new(Self {
            id: id.to_string(),
            name: name.to_string(),
            binded_opposite_odds: RefCell::new(None),
        })
    }

    fn bind(&self, a: &Rc<A>) {
        *self.binded_opposite_odds.borrow_mut() = Some(Rc::clone(a));
    }
}

fn main() {
    let a = A::new("1111", "zhangsan");
    let b = B::new("zzzz", "lisi");

    a.bind(&b);
    b.bind(&a);

    // 使用引用访问 binded_opposite_odds
    let bbb = a.binded_opposite_odds.borrow();
    if let Some(ref bb) = *bbb {
        println!("B's id: {}", bb.id);
        println!("B's name: {}", bb.name);
    }

    println!("\n\n\n");

    let aaa = b.binded_opposite_odds.borrow();
    if let Some(ref aa) = *aaa {
        println!("A's id: {}", aa.id);
        println!("A's name: {}", aa.name);
    }

    // 不可以直接打印循环引用对象, 会导致程序崩溃(堆栈溢出).
    // println!("{:?}", a);
}
