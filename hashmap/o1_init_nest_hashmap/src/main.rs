use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    name: String,
    age: String,
}

type NestMap = HashMap<String, HashMap<String, HashMap<String, HashMap<String, Person>>>>;

fn main() {
    let mut nm = NestMap::new();

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
