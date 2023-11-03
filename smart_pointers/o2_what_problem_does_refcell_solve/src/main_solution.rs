use std::cell::RefCell;
use std::collections::HashMap;

struct CachedCalculator {
    cache: RefCell<HashMap<i32, i32>>,
}

impl CachedCalculator {
    fn new() -> Self {
        CachedCalculator {
            cache: RefCell::new(HashMap::new()),
        }
    }

    fn calculate(&self, value: i32) -> i32 {
        // 首先检查缓存
        if let Some(&cached_result) = self.cache.borrow().get(&value) {
            return cached_result;
        }

        // 计算（这里简化为 value * value）
        let result = value * value;

        // 存入缓存
        self.cache.borrow_mut().insert(value, result);

        result
    }
}

fn main() {
    let calculator = CachedCalculator::new();
    assert_eq!(calculator.calculate(2), 4);
    assert_eq!(calculator.calculate(3), 9);
    assert_eq!(calculator.calculate(2), 4); // 这里使用缓存
}
