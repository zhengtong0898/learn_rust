use std::collections::HashMap;

struct CachedCalculator {
    cache: HashMap<i32, i32>,
}

impl CachedCalculator {
    fn new() -> Self {
        CachedCalculator {
            cache: HashMap::new(),
        }
    }

    // 2. self本身是不可变的, 因此不能修改内部的值.
    fn calculate(&self, value: i32) -> i32 {
        // 首先检查缓存
        if let Some(&cached_result) = self.cache.get(&value) {
            return cached_result;
        }

        // 计算（这里简化为 value * value）
        let result = value * value;

        // 尝试存入缓存
        self.cache.insert(value, result); // 这里会导致编译错误

        result
    }
}

fn main() {
    // 1. 不可变的变量.
    let calculator = CachedCalculator::new();
    assert_eq!(calculator.calculate(2), 4);
    assert_eq!(calculator.calculate(3), 9);
    assert_eq!(calculator.calculate(2), 4);
}
