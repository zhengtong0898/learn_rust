// 多态

// 定义两个接口, 都要求实现接口的对象必须包含fly方法.
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

// Human 实现了Pilot接口的fly方法, 实现了Wizard接口的fly方法.
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// Human 也实现了自己的fly方法, 跟接口无关, 只是名字刚好相同.
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    // 直接调用
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("--------------------------");

    // 类型转换调用
    <Human as Pilot>::fly(&person);
}
