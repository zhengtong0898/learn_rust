/*

注意事项:
1. hello_macro 是一个独立的 lib 库

2. hello_macro_derive 也是一个独立的 lib 库

3. 按照约定，对于名为foo的crate，自定义派生程序宏crate被称为foo_derive,
   所以 hello_macro 的自定义派生过程宏因为是 hello_macro_derive.

4. 由于它们两从逻辑上是强关联的, 当我们修改 hello_macro 中的 trait 时,
   也需要去修改 hello_macro_derive 中的程序宏实现,
   因此将 hello_macro_derive 放在 hello_macro 库目录中是一种非常恰当的做法.

*/

// 这是一个普通的trait定义.
// 从 Cargo.toml 来看, 这也是一个普通的 lib 库.
pub trait HelloMacro {
    fn hello_macro();
}
