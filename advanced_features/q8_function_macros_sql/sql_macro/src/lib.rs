extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/*
Function-like macros / 函数风格的宏

允许在代码中直接使用 `函数风格的宏`, 这极大的增加了代码的灵活度.

`函数风格的宏`接收TokenStream作为入参, 这意味着可以支持不限数量的参数作为输入, 只要能够正确的从流中解析出值来即可.

*/

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    // input是一个LitStr的结构体, 通过input.value()来提取所有字符串.
    let content = input.value();

    // 条件判断, 生成不同的字符模板
    if content.contains("SELECT") {
        quote! {
            "This is a SELECT statement."
        }
    } else {
        quote! {
            "This is not a SELECT statement."
        }
    }
    .into()
}
