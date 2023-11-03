#![allow(unused)]
fn main() {
    use std::collections::HashMap;
    use std::hash::Hash;

    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash, // 这行是什么意思?
        V: Default,           // 这行是什么意思?
    {
        // 这个代码的目的是:
        // 如果key存在, 则返回value.
        // 如果key不存在, 则将key写入到map.
        //
        // 遇到的问题:
        // 1. map.get_mut(&key) 产生了一次可变引用, rust认为此次可变引用要在match匹配结束后才会释放.
        // 2. 所以在match模式匹配的作用域内, 使用map.insert是再次产生一次可变引用, 这就报错了.
        match map.get_mut(&key) {
            Some(value) => value,
            None => {
                map.insert(key.clone(), V::default());
                map.get_mut(&key).unwrap()
            }
        }
    }
}
