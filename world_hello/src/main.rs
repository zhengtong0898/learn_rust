#[allow(unused_variables)]
fn main() {
    let x = 5;
    let equal_to_x = || println!("{}", x);
    println!("{}", x);
    equal_to_x();
    println!("{}", x);
}
