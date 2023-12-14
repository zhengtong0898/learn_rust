use std::any::Any;
use std::sync::Arc;

// 需要手动声明当前trait是一个 Send + Sync ,
// 这样编译期就会检查当前trait的所有实现是否存在 not Send 的数据.
trait MyTrait: Send + Sync + Any {
    fn as_any(&self) -> Arc<dyn Any + Send + Sync>;
}

#[derive(Clone)]
struct Person {
    name: String,
    age: i64,
}

impl MyTrait for Person {
    fn as_any(&self) -> Arc<dyn Any + Send + Sync> {
        Arc::new(self.clone())
    }
}

#[tokio::main]
async fn main() {
    let person_trait: Arc<dyn MyTrait> = Arc::new(Person {
        name: "zhangsan".to_string(),
        age: 20,
    });

    // 多线程异步共享: 只读
    let person_trait_clone = person_trait.clone();
    let handler1 = tokio::spawn(async move {
        if let Some(person) = person_trait_clone.as_any().downcast_ref::<Person>() {
            println!("person.name: {}, person.age: {}", person.name, person.age);
        }
    });

    // 多线程异步共享: 只读
    let person_trait_clone = person_trait.clone();
    let handler2 = tokio::spawn(async move {
        if let Some(person) = person_trait_clone.as_any().downcast_ref::<Person>() {
            println!("person.name: {}, person.age: {}", person.name, person.age);
        }
    });

    let _ = handler1.await;
    let _ = handler2.await;
}
