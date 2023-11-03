use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = Some(
    //     Node {
    //         value: 5,
    //         parent: RefCell { value: (Weak) },
    //         children: RefCell {
    //             value: [
    //                 Node {
    //                     value: 3,
    //                     parent: RefCell { value: (Weak) },
    //                     children: RefCell { value: [] }
    //                 }
    //             ]
    //         }
    //     }
    // )
}
