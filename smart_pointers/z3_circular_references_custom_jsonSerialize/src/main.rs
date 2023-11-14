use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::cell::RefCell;
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

impl Serialize for A {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;

        let mut opposite = None;
        if let Some(o) = self.binded_opposite_odds.borrow().as_ref() {
            opposite = Some(B {
                id: o.id.to_string(),
                name: o.name.to_string(),
                binded_opposite_odds: RefCell::new(None),
            });
        }
        match opposite {
            Some(o) => {
                state.serialize_field("binded_opposite_odds_id", &o)?;
            }
            None => {
                state.serialize_field("binded_opposite_odds_id", "")?;
            }
        }

        state.end()
    }
}

impl Serialize for B {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;

        let mut opposite = None;
        if let Some(o) = self.binded_opposite_odds.borrow().as_ref() {
            opposite = Some(B {
                id: o.id.to_string(),
                name: o.name.to_string(),
                binded_opposite_odds: RefCell::new(None),
            });
        }
        match opposite {
            Some(o) => {
                state.serialize_field("binded_opposite_odds_id", &o)?;
            }
            None => {
                state.serialize_field("binded_opposite_odds_id", "")?;
            }
        }

        state.end()
    }
}

fn main() {
    let a = A::new("1111", "zhangsan");
    let b = B::new("zzzz", "lisi");

    a.bind(&b);
    b.bind(&a);

    // 序列化并打印
    let serialized_a = serde_json::to_string(&*a).unwrap();
    println!("{}", serialized_a);

    let serialized_b = serde_json::to_string(&*b).unwrap();
    println!("{}", serialized_b);
}
