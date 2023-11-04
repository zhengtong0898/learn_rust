use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
struct Parent {
    id: u32,
    child: RefCell<Option<Rc<Child>>>, // 这里使用Option的意思是说这个值可能存在或不存在.
}

#[allow(dead_code)]
#[derive(Debug)]
struct Child {
    id: u32,
    parent: RefCell<Option<Rc<Parent>>>,
}

fn main() {
    let parent = Rc::new(Parent {
        id: 1,
        child: RefCell::new(None),
    });

    let child = Rc::new(Child {
        id: 2,
        parent: RefCell::new(None),
    });

    // 建立父子关系
    //  parent.child              的类型是: RefCell<Option<Rc<Child>>>
    //  parent.child.borrow_mut() 的类型是: RefMut<Option<Rc<Child>>>
    // *parent.child.borrow_mut() 的类型是: Option<Rc<Child>>            // 解引用, 在这里指的是解RefMut这个引用
    *parent.child.borrow_mut() = Some(child.clone());
    *child.parent.borrow_mut() = Some(parent.clone());

    // 打印结构体，看看它们是否正确关联
    // println!("{:?}", parent);       // Error: 无限循环的打印直到堆栈溢出.
    // println!("{:?}", child);

    // 打破循环引用
    *parent.child.borrow_mut() = None;
    *child.parent.borrow_mut() = None;
}
