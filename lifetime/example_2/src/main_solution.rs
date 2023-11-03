#![allow(unused)]
fn main() {
    use std::collections::HashMap;
    use std::hash::Hash;

    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash, // 这行是什么意思?
        V: Default,           // 这行是什么意思?
    {
        if !map.contains_key(&key) {
            map.insert(key.clone(), V::default());
        }

        map.get_mut(&key).unwrap()
    }
}
