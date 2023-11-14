use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

// 定义结构体 A
struct A {
    id: String,
    name: String,
    binded_opposite_odds: RefCell<Option<Rc<B>>>,
}

// 定义结构体 B
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

impl fmt::Debug for A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut opposite = None;
        if let Some(o) = self.binded_opposite_odds.borrow().as_ref() {
            opposite = Some(B {
                id: o.id.to_string(),
                name: o.name.to_string(),
                binded_opposite_odds: RefCell::new(None),
            });
        }

        match opposite {
            Some(x) => f
                .debug_struct("")
                .field("id", &self.id)
                .field("name", &self.name)
                .field("binded_opposite_odds", &x)
                .finish(),
            None => f
                .debug_struct("")
                .field("id", &self.id)
                .field("name", &self.name)
                .finish(),
        }
    }
}

impl fmt::Debug for B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut opposite = None;
        if let Some(o) = self.binded_opposite_odds.borrow().as_ref() {
            opposite = Some(A {
                id: o.id.to_string(),
                name: o.name.to_string(),
                binded_opposite_odds: RefCell::new(None),
            });
        }

        match opposite {
            Some(x) => f
                .debug_struct("")
                .field("id", &self.id)
                .field("name", &self.name)
                .field("binded_opposite_odds", &x)
                .finish(),
            None => f
                .debug_struct("")
                .field("id", &self.id)
                .field("name", &self.name)
                .finish(),
        }
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

    println!();

    let aaa = b.binded_opposite_odds.borrow();
    if let Some(ref aa) = *aaa {
        println!("A's id: {}", aa.id);
        println!("A's name: {}", aa.name);
    }

    println!();

    // 通过自定义Debug, 避免在打印循环引用对象时出现堆栈溢出问题
    println!("{:?}", a);
    println!("{:?}", b);
}
