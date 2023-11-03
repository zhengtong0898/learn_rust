use proc_macro::TokenStream as ProcTokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use syn;

/*
derive macro trait / 派生(自动继承)宏trait(的方法)

只能作用在 struct 或 enum 上, 用于为 struct 或 enum 自动派生 宏trait 的方法.
*/

// #[proc_macro_derive(HelloMacro)]
// 告诉Rust编译期, 当遇到 #[derive(HelloMacro)] 这样的代码时应该调用这个函数.
#[proc_macro_derive(HelloMacro)]
// Rust 的 procedural macros（过程宏）在定义时，通常需要处理的数据类型是 TokenStream,
// input 是传入宏的原始代码表示，其类型为 TokenStream .
// 你需要解析这个 TokenStream 来理解和处理它，最后再产生一个新的 TokenStream 作为输出
pub fn hello_macro_derive(input: ProcTokenStream) -> ProcTokenStream {
    // `quote!`宏, 提供了一个类似于模板的语法, 你可以插入变量、重复代码片段, 并生成Rust代码.

    // 打印 ProcTokenStream 对象.
    // quote! { #input2 }; 中的 #input2 的意思 等同于 shell 中的 $input2, 即: 使用这个变量.
    //     let var_name = "example";
    //     let generated_code = quote! {
    //         let #var_name = "This is an example.";
    //     };
    // 这是一种非常高级的特性, 允许变量的值作为变量名来使用, 这是我之前在python中一直无法做到的事情.
    let input2: TokenStream = TokenStream::from(input);
    let tokens = quote! { #input2 };
    eprintln!("hello: {}", tokens);

    let ast = syn::parse2(input2).unwrap();
    impl_hello_macro(&ast)
}

// 这是一个普通的私有函数
fn impl_hello_macro(ast: &syn::DeriveInput) -> ProcTokenStream {
    // 将标识符赋值给变量name
    let name = &ast.ident;

    // 使用 quote! 来定义要生成的代码.
    // 这里要生成的是一个 HelloMacro 实现.
    // 如果ast传递进来的是 struct Pancakes , 那么这里就是 impl HelloMacro for Pancakes { fn hello_macro() { ... } }
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}
