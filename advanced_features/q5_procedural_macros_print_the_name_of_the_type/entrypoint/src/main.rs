use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[allow(dead_code)]
#[derive(HelloMacro)]
struct Pancakes {
    name: String,
}

fn main() {
    Pancakes::hello_macro();
}
