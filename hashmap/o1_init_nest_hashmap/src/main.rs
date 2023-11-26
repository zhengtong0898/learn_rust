use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: String,
}

type NestMap = HashMap<String, HashMap<String, HashMap<String, HashMap<String, Person>>>>;

fn main() {
    let mut nm = NestMap::new();

    // hashmap 的 entry 函数返回值是 Entry 结构体, 它有两个属性: Occupied、Vacant.
    //     Occupied: key已存在.
    //     Vacant: key不存在.
    //
    // or_insert_with 函数的含义是,
    //     如果 Entry 是 Occupied 时(key已存在), 返回 nm["a"] 对应的 Hashmap 对象.
    //     如果 Entry 时 Vacant 时(key不存在), 执行 Hashmap::new() 创建并返回 Hashmap 对象.
    //
    // insert 函数的含义是:
    //     向 nm["a"]["b"]["c"] 插入一个 k, v 键值对.
    nm.entry("a".to_string())
        .or_insert_with(HashMap::new)
        .entry("b".to_string())
        .or_insert_with(HashMap::new)
        .entry("c".to_string())
        .or_insert_with(HashMap::new)
        .insert(
            "d".to_string(),
            Person {
                name: "zt".to_string(),
                age: "11".to_string(),
            },
        );

    println!("Hello, world!: {:?}", nm);
}
