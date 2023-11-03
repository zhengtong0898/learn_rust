use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

// 这里不采用默认泛型类型, 而是指定泛型类型为Triangle.
impl Add<Triangle> for Point {
    type Output = Point;

    fn add(self, other: Triangle) -> Point {
        Point {
            x: self.x + other.a,
            y: self.y + other.b,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Triangle { a: 2, b: 3, c: 1 },
        Point { x: 3, y: 3 }
    );
}
