use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 在源码中 `Add trait` 的签名是: pub trait Add<Rhs = Self> {}
// 其中 `Rhs` 等同于泛型中的 `T`, 而 `Self` 是默认泛型类型.
//
// 在当前的实现中 `Add` 后面没有跟任何具体类型, 那么就采用默认泛型类型`Self`.
// 问题: 如何在 `Add` 后面跟具体类型? 查看main_triangle.rs.
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
